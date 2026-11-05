use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::{json, Value};
use solana_keypair::Keypair;
use solana_signer::Signer;
use zolana_client::rpc::{OutputContext, OutputSlot};
use zolana_interface::instruction::{CircuitId, InputUtxo, TransactProof, TreeContext};
use zolana_program::instruction::Transact;
use zolana_transaction::{AssetRegistry, History, Mint, Utxo};

use super::*;
use crate::{wallet::history::BlockTime, ActivityKind, WalletConfig};

/// One asset's private payments and public withdrawals in a spend.
type Leg<'a> = (
    Mint,
    &'a [(&'a MobileWallet, u64)],
    &'a [(&'a MobileWallet, u64)],
);

fn config() -> WalletConfig {
    WalletConfig {
        rpc_url: "https://rpc.example".into(),
        indexer_url: "https://indexer.example".into(),
        proving_key_dir: "/unused".into(),
        proving_key_url: None,
        proving_key_catalog: None,
        proving_keys_from_prover: None,
        proving: None,
        prover_url: None,
        require_prover_tee: None,
        prover_tee_policy: None,
        prover_tee_network: None,
        prover_sync_only: None,
        max_spend_inputs: None,
        reuse_notes_ms: None,
        mints: None,
        priority_fee_lamports: None,
    }
}

fn wallet(
    signer: &Keypair,
    handler: impl Fn(&str, &Value) -> Value + Send + Sync + 'static,
) -> (MobileWallet, Arc<Mutex<Vec<String>>>) {
    let asked = Arc::new(Mutex::new(Vec::new()));
    let calls = Arc::clone(&asked);
    let (transport, _) = crate::transport::tests::fake(move |request| {
        let request: Value = serde_json::from_slice(&request.body).unwrap();
        let method = request["method"].as_str().unwrap();
        calls.lock().unwrap().push(method.to_owned());
        let mut answer = handler(method, &request["params"]);
        answer["jsonrpc"] = json!("2.0");
        answer["id"] = request["id"].clone();
        crate::transport::tests::ok(&answer.to_string())
    });
    let key = signer.pubkey().to_string();
    let signature = signer.sign_message(&derivation_message(key.clone()).unwrap());
    (
        MobileWallet::open(config(), key, signature.as_ref().to_vec(), transport).unwrap(),
        asked,
    )
}

fn identity(signer: &Keypair) -> MobileWallet {
    wallet(signer, |method, _| panic!("unexpected request: {method}")).0
}

struct Fixture {
    signature: Signature,
    chain: Value,
    events: Vec<ShieldedTransaction>,
    registry: AssetRegistry,
    notes: Vec<WalletUtxo>,
    records: HashMap<Pubkey, zolana_user_registry_interface::UserRecord>,
    /// The spend's outputs, as its prepared transaction keeps them.
    openings: Vec<OutputOpening>,
}

impl Fixture {
    /// Real encrypted default-ring spend, with known source notes only in the
    /// fixture. The lookup receives neither those notes nor any history page.
    fn spend(
        sender: &MobileWallet,
        recipient: &MobileWallet,
        private: u64,
        withdrawal: u64,
        assets: &[Mint],
    ) -> Self {
        Self::spend_with_cache(sender, recipient, private, withdrawal, assets, false)
    }

    fn spend_with_cache(
        sender: &MobileWallet,
        recipient: &MobileWallet,
        private: u64,
        withdrawal: u64,
        assets: &[Mint],
        cached: bool,
    ) -> Self {
        Self::spend_to_outputs(
            sender,
            &[(recipient, private)],
            &[(recipient, withdrawal)],
            assets,
            cached,
        )
    }

    fn spend_to_outputs(
        sender: &MobileWallet,
        private: &[(&MobileWallet, u64)],
        withdrawals: &[(&MobileWallet, u64)],
        assets: &[Mint],
        cached: bool,
    ) -> Self {
        let legs: Vec<Leg> = assets
            .iter()
            .map(|asset| (*asset, private, withdrawals))
            .collect();
        Self::spend_legs(sender, &legs, cached)
    }

    /// One spend paying each leg's outputs in that leg's asset.
    fn spend_legs(sender: &MobileWallet, legs: &[Leg], cached: bool) -> Self {
        let payer = sender.owner;
        let mut registry = AssetRegistry::default();
        for (mint, _, _) in legs {
            if mint.asset != SOL_MINT {
                registry.insert(mint.asset_id, mint.asset).unwrap();
            }
        }
        let mut notes = Vec::new();
        for (index, (asset, _, _)) in legs.iter().enumerate() {
            let utxo = Utxo {
                owner: sender.address.signing_pubkey,
                asset: *asset,
                amount: 100,
                blinding: [index as u8 + 1; 32],
                ring_program_id: None,
                data: Default::default(),
            };
            let hash = utxo
                .hash(&sender.address.nullifier_pubkey, &[0; 32], &[0; 32], 0)
                .unwrap();
            let nullifier = utxo.nullifier(&hash, &sender.nullifier_key).unwrap();
            notes.push(WalletUtxo {
                utxo,
                nullifier_pubkey: sender.address.nullifier_pubkey,
                utxo_hash: hash,
                nullifier,
                tx_viewing_key: None,
                data_hash: None,
                ring_data_hash: None,
                tree_id: 0,
                leaf_index: index as u64,
                latest_tree_id: None,
                slot: 1,
                tx_signature: Signature::from([1; 64]),
                slot_index: index as u32,
            });
        }
        let mut transaction = ConfidentialTransaction::new(notes.clone(), payer).unwrap();
        let mut settlements = Vec::new();
        for (asset, private, withdrawals) in legs {
            for &(recipient, amount) in private.iter().filter(|(_, amount)| *amount > 0) {
                if asset.asset == SOL_MINT {
                    transaction
                        .transfer_sol(&recipient.address, amount)
                        .unwrap();
                } else {
                    transaction
                        .transfer(&recipient.address, asset.asset, amount)
                        .unwrap();
                }
            }
            for &(recipient, amount) in withdrawals.iter().filter(|(_, amount)| *amount > 0) {
                settlements.push(
                    transaction
                        .withdraw_to(
                            asset.asset,
                            amount,
                            recipient.owner,
                            (asset.asset != SOL_MINT).then(pda::spl_token_program_id),
                        )
                        .unwrap(),
                );
            }
        }
        let inputs = transaction.encrypt(&sender.keys).unwrap();
        let tree_id = inputs.output_tree_id;
        let openings = inputs
            .output_utxos
            .iter()
            .map(|output| OutputOpening::from_output(output, tree_id))
            .collect();
        // Read back only wire-visible data, just as the indexer does.
        let data = TransactIxData {
            expiry_unix_ts: u64::MAX,
            tx_viewing_pk: inputs.external_data.tx_viewing_pk,
            salt: inputs.external_data.salt,
            interface_transfers: inputs
                .external_data
                .interface_transfers
                .iter()
                .map(|leg| leg.interface_transfer())
                .collect(),
            data_hash: None,
            ring_data_hash: None,
            outputs: inputs.external_data.outputs.clone(),
            messages: vec![],
            private_tx_hash: [0; 32],
            circuit: CircuitId::ConfidentialSigner(
                inputs.input_utxos.len() as u8,
                inputs.output_utxos.len() as u8,
                3,
            ),
            proof: TransactProof::zeroed(),
            inputs: inputs
                .input_utxos
                .iter()
                .filter(|input| !input.compact)
                .map(|input| InputUtxo {
                    nullifier_hash: input.nullifier(),
                    tree_index: 0,
                })
                .collect(),
            tree_contexts: vec![TreeContext {
                utxo_tree_root_index: 0,
                nullifier_tree_root_index: 0,
            }],
        };
        let mut builder = Transact {
            payer,
            input_trees: vec![pda::tree(0)],
            output_tree: pda::tree(tree_id),
            owner_signers: vec![sender.owner],
            interface_transfer_accounts: settlements,
            data,
        };
        let ix = if cached {
            builder.data.circuit = CircuitId::ConfidentialSignerCached(
                inputs.input_utxos.len() as u8,
                inputs.output_utxos.len() as u8,
                3,
                zolana_interface::verifying_keys::CacheAccess {
                    read_bitmap: 1,
                    write_slots: zolana_interface::verifying_keys::CacheAccess::NO_WRITES,
                },
            );
            builder.instruction_with_cache_read(pda::cache(&payer, 9).0)
        } else {
            builder.instruction()
        };
        let signature = Signature::from([12; 64]);
        let message = compile_message(
            &payer,
            std::slice::from_ref(&ix),
            Default::default(),
            ComputeBudgetConfig::new(1_000_000),
        )
        .unwrap();
        let signed = VersionedTransaction {
            signatures: vec![signature],
            message,
        };
        let chain = json!({"slot": 10, "blockTime": 0, "version": 1,
            "transaction": {"signatures": [signature.to_string()], "message": {
                "header": signed.message.header(),
                "accountKeys": signed.message.static_account_keys().iter().map(ToString::to_string).collect::<Vec<_>>(),
                "recentBlockhash": signed.message.recent_blockhash().to_string(),
                "instructions": signed.message.instructions().iter().map(|ix| solana_transaction_status_client_types::UiCompiledInstruction::from(ix, None)).collect::<Vec<_>>(),
                "addressTableLookups": []
            }},
            "meta": {"err": null, "status": {"Ok": null}, "fee": 5000,
                "preBalances": [], "postBalances": [], "innerInstructions": [], "logMessages": [],
                "preTokenBalances": [], "postTokenBalances": [], "rewards": []}
        });
        let event = ShieldedTransaction {
            slot: 10,
            tx_signature: signature,
            event_index: Some(0),
            tx_viewing_pk: Some(
                zolana_keypair::X25519Pubkey::from_bytes(builder.data.tx_viewing_pk).unwrap(),
            ),
            salt: Some(builder.data.salt),
            nullifiers: builder
                .data
                .inputs
                .iter()
                .map(|input| input.nullifier_hash)
                .collect(),
            output_slots: builder
                .data
                .outputs
                .iter()
                .enumerate()
                .map(|(index, output)| OutputSlot {
                    view_tag: fetch_tag(&output.owner_tag, |i| {
                        ix.accounts
                            .get(usize::from(i))
                            .map(|account| account.pubkey.to_bytes())
                    })
                    .unwrap(),
                    output_context: OutputContext {
                        hash: output.utxo_hash,
                        tree_id,
                        leaf_index: 100 + index as u64,
                    },
                    payload: output.data.clone().unwrap_or_default(),
                })
                .collect(),
            messages: vec![],
            proofless: false,
            ring_config: None,
            ring_program_id: None,
        };
        let records = legs
            .iter()
            .flat_map(|(_, private, _)| private.iter())
            .map(|(recipient, _)| {
                let record = zolana_user_registry_interface::UserRecord {
                    owner: recipient.owner,
                    bump: 0,
                    nullifier_pubkey: recipient.address.nullifier_pubkey,
                    viewing_pubkey: *recipient.address.viewing_pubkey.as_bytes(),
                    merge_encryption: MergeEncryptionScheme::Disabled,
                };
                (recipient.owner, record)
            })
            .collect();
        Self {
            signature,
            chain,
            events: vec![event],
            registry,
            notes,
            records,
            openings,
        }
    }

    /// A prepared spend holding this fixture's output openings.
    fn prepared(&self, signer: &Keypair) -> PreparedTransaction {
        let mut pending = crate::wallet::tests::unsigned(signer);
        pending.outputs = self.openings.clone();
        pending
    }

    fn scan(&self, owner: &MobileWallet) -> UtxoScanResult {
        let decoded = decrypt(&owner.keys, &self.events, &self.registry).unwrap();
        let mut utxos = self.notes.clone();
        utxos.extend(verify_owned(&owner.keys, &decoded).unwrap().utxos);
        UtxoScanResult {
            asset_registry: self.registry.clone(),
            spendable: Default::default(),
            history: History {
                transactions: self.events.clone(),
                utxos,
                view_tags: [owner
                    .address
                    .signing_pubkey
                    .confidential_view_tag()
                    .unwrap()]
                .into_iter()
                .collect(),
                ..Default::default()
            },
        }
    }

    fn indexer(&self, slot: u64) -> Value {
        json!({"context": {"slot": slot, "blockTime": 0}, "transactions": self.events.iter().enumerate().map(|(index, tx)| json!({
            "eventIndex": index, "transaction": {
                "slot": tx.slot, "txSignature": tx.tx_signature.to_string(), "eventIndex": tx.event_index,
                "txViewingPk": tx.tx_viewing_pk.map(|key| STANDARD.encode(key.as_bytes())),
                "salt": tx.salt.map(|salt| STANDARD.encode(salt)), "proofless": tx.proofless,
                "nullifiers": tx.nullifiers.iter().map(|hash| zolana_api::Hash::from(*hash).to_base58()).collect::<Vec<_>>(),
                "messages": [], "outputSlots": tx.output_slots.iter().map(|slot| json!({
                    "viewTag": zolana_api::Hash::from(slot.view_tag).to_base58(), "payload": STANDARD.encode(&slot.payload),
                    "outputContext": {"hash": zolana_api::Hash::from(slot.output_context.hash).to_base58(),
                        "tree": pda::tree(slot.output_context.tree_id).to_string(), "treeId": slot.output_context.tree_id,
                        "leafIndex": slot.output_context.leaf_index}
                })).collect::<Vec<_>>()
            }
        })).collect::<Vec<_>>()})
    }

    fn open(self, signer: &Keypair) -> (MobileWallet, Arc<Mutex<Vec<String>>>) {
        self.open_with_registry(signer, true)
    }

    fn open_with_registry(
        self,
        signer: &Keypair,
        preload: bool,
    ) -> (MobileWallet, Arc<Mutex<Vec<String>>>) {
        let registry = self.registry.clone();
        let indexed = self.indexer(10);
        let (wallet, calls) = wallet(signer, move |method, params| match method {
            "getShieldedTransactionsBySignature" => json!({"result": indexed}),
            "getTransaction" => json!({"result": self.chain}),
            "getProgramAccounts" => {
                json!({"result": self.notes.iter().filter(|note| note.utxo.asset.asset != SOL_MINT).map(|note| {
                let mint = note.utxo.asset;
                json!({"pubkey": pda::spl_asset_registry(&mint.asset).to_string(), "account": {
                    "owner": PROGRAM_ID_PUBKEY.to_string(), "lamports": 1000000, "executable": false, "rentEpoch": 0,
                    "data": [STANDARD.encode(zolana_interface::state::SplAssetRegistry::account_bytes(mint.asset, mint.asset_id)), "base64"]
                }})
            }).collect::<Vec<_>>()})
            }
            "getUserRecords" => {
                let records = params["owners"].as_array().unwrap().iter().map(|owner| {
                    let owner = Pubkey::from_str(owner.as_str().unwrap()).unwrap();
                    self.records.get(&owner).map(|record| json!({"owner": record.owner.to_string(),
                        "nullifierPubkey": zolana_api::Hash::from(record.nullifier_pubkey).to_base58(),
                        "viewingPubkey": STANDARD.encode(record.viewing_pubkey), "mergeEncryption": 0}))
                }).collect::<Vec<_>>();
                json!({"result": {"context": {"slot": 10, "blockTime": 0}, "records": records}})
            }
            _ => panic!("lookup must not fetch history: {method}"),
        });
        if preload {
            wallet.assets().remember_registry(registry);
        }
        (wallet, calls)
    }
}

#[test]
fn cold_lookup_matches_history_for_sends_withdrawals_and_multiple_assets() {
    let sender = Keypair::new();
    let recipient = Keypair::new();
    let owner = identity(&sender);
    let recipient_wallet = identity(&recipient);
    for (private, withdrawal) in [(20, 0), (0, 100), (10, 20)] {
        for (assets, cached) in [
            (vec![Mint::SOL], false),
            (vec![Mint::SOL], true),
            (vec![Mint::SOL, Mint::new(Pubkey::new_unique(), 7)], false),
            (vec![Mint::SOL, Mint::new(Pubkey::new_unique(), 7)], true),
        ] {
            let fixture = Fixture::spend_with_cache(
                &owner,
                &recipient_wallet,
                private,
                withdrawal,
                &assets,
                cached,
            );
            let scan = fixture.scan(&owner);
            let mut expected = activity::entries(&scan.history, &scan.asset_registry).unwrap();
            for entry in &mut expected {
                entry.block_time = Some(0);
            }
            let confirmed: Confirmed = serde_json::from_value(fixture.chain.clone()).unwrap();
            let groups = ConfirmedInstructionGroups::try_from(confirmed)
                .expect("fixture instruction groups");
            assert!(
                transact_context(&fixture.events[0], &groups).is_some(),
                "fixture event matches its instruction"
            );
            let signature = fixture.signature.to_string();
            let (wallet, asked) = fixture.open(&sender);
            let result = wallet.transaction(signature).unwrap();
            assert_eq!(
                result.status,
                Status::Ready,
                "private={private}, withdrawal={withdrawal}, assets={assets:?}, calls={:?}",
                asked.lock().unwrap()
            );
            assert_eq!(result.entries, expected);
            assert!(!result.entries.is_empty());
            assert_eq!(result.indexed_slot, 10);
            assert_eq!(result.block_time, Some(0));
            assert_eq!(result.fee, Some(5000));
            let expected_outputs = assets
                .iter()
                .flat_map(|asset| {
                    let private = (private > 0).then(|| TransactionOutput {
                        recipient: recipient_wallet.owner.to_string(),
                        mint: mint_name(&asset.asset),
                        amount: private,
                        is_public: false,
                    });
                    let public = (withdrawal > 0).then(|| TransactionOutput {
                        recipient: if asset.asset == SOL_MINT {
                            recipient_wallet.owner
                        } else {
                            pda::associated_token_address_with_program(
                                &recipient_wallet.owner,
                                &asset.asset,
                                &pda::spl_token_program_id(),
                            )
                        }
                        .to_string(),
                        mint: mint_name(&asset.asset),
                        amount: withdrawal,
                        is_public: true,
                    });
                    private.into_iter().chain(public)
                })
                .collect::<Vec<_>>();
            assert_eq!(result.outputs.len(), expected_outputs.len());
            for output in expected_outputs {
                assert!(result.outputs.contains(&output), "missing {output:?}");
            }
            assert!(asked.lock().unwrap().iter().all(|method| [
                "getTransaction",
                "getShieldedTransactionsBySignature",
                "getUserRecords"
            ]
            .contains(&method.as_str())));
        }
    }
}

#[test]
fn payments_variable_collector_fees_and_withdrawals_keep_each_output() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let collector = identity(&Keypair::new());
    // A withdrawal with a private fee has only the collector's private
    // output. A mixed payment has two distinct outputs to the same recipient.
    for fee in [1, 7] {
        for send in [false, true] {
            let mut private = vec![(&collector, fee)];
            if send {
                private.extend([(&recipient, 20), (&recipient, 3)]);
            }
            let fixture = Fixture::spend_to_outputs(
                &owner,
                &private,
                &[(&recipient, 30), (&owner, 5)],
                &[Mint::SOL],
                false,
            );
            let signature = fixture.signature.to_string();
            let mut expected = private
                .iter()
                .map(|(wallet, amount)| TransactionOutput {
                    recipient: wallet.owner.to_string(),
                    mint: None,
                    amount: *amount,
                    is_public: false,
                })
                .collect::<Vec<_>>();
            expected.extend([
                TransactionOutput {
                    recipient: recipient.owner.to_string(),
                    mint: None,
                    amount: 30,
                    is_public: true,
                },
                TransactionOutput {
                    recipient: owner.owner.to_string(),
                    mint: None,
                    amount: 5,
                    is_public: true,
                },
            ]);
            let (wallet, _) = fixture.open(&signer);
            let result = wallet.transaction(signature).unwrap();
            assert_eq!(result.status, Status::Ready);
            assert_eq!(result.outputs, expected);
            assert_eq!(result.entries.len(), 1);
            assert_eq!(
                result.entries[0].amount,
                fee + 35 + if send { 23 } else { 0 }
            );
            assert_eq!(
                result.fee,
                Some(5000),
                "network fee is separate from collector fee"
            );
        }
    }
}

#[test]
fn activity_times_are_cached_per_slot_and_lookup_seeds_the_same_cache() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let fixture = Fixture::spend(
        &owner,
        &recipient,
        20,
        0,
        &[Mint::SOL, Mint::new(Pubkey::new_unique(), 7)],
    );
    let scan = fixture.scan(&owner);
    let (reader, asked) = wallet(&signer, |method, params| {
        assert_eq!(method, "getBlockTime");
        assert_eq!(params, &json!([10]));
        json!({"result": 1_800_000_000})
    });
    for _ in 0..2 {
        let entries = super::super::history::entries(&reader, &scan).unwrap();
        assert_eq!(entries.len(), 2);
        for entry in entries {
            assert_eq!(entry.block_time, Some(1_800_000_000));
            assert_eq!(entry.recipients, vec![recipient.owner.to_string()]);
        }
    }
    assert_eq!(*asked.lock().unwrap(), vec!["getBlockTime"]);

    let signature = fixture.signature.to_string();
    let (reader, asked) = fixture.open(&signer);
    let result = reader.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Ready);
    let entries = super::super::history::entries(&reader, &scan).unwrap();
    assert_eq!(entries, result.entries);
    assert!(!asked
        .lock()
        .unwrap()
        .iter()
        .any(|method| method == "getBlockTime"));
}

#[test]
fn activity_timestamps_follow_slots_rather_than_transaction_order() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let fixture = Fixture::spend(&owner, &recipient, 20, 0, &[Mint::SOL]);
    let mut scan = fixture.scan(&owner);
    let first = scan.history.transactions[0].clone();
    scan.history.transactions = [12, 10, 12, 11]
        .into_iter()
        .enumerate()
        .map(|(i, slot)| {
            let mut tx = first.clone();
            tx.slot = slot;
            tx.tx_signature = Signature::from([i as u8; 64]);
            tx
        })
        .collect();
    let (reader, asked) = wallet(&signer, |method, params| {
        assert_eq!(method, "getBlockTime");
        json!({"result": 1_800_000_000 + params[0].as_i64().unwrap()})
    });
    let entries = super::super::history::entries(&reader, &scan).unwrap();
    assert_eq!(entries.len(), 4);
    for entry in entries {
        assert_eq!(entry.block_time, Some(1_800_000_000 + entry.slot as i64));
    }
    assert_eq!(
        asked.lock().unwrap().len(),
        3,
        "one request per distinct slot"
    );
}

#[test]
fn unavailable_activity_times_preserve_rows_and_are_retried_after_a_growing_wait() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let fixture = Fixture::spend(&owner, &recipient, 20, 0, &[Mint::SOL]);
    let scan = fixture.scan(&owner);
    for miss in [
        json!({"result": null}),
        json!({"error": {"code": -32009, "message": "slot unavailable"}}),
    ] {
        let calls = AtomicUsize::new(0);
        let (reader, asked) = wallet(&signer, move |method, _| {
            assert_eq!(method, "getBlockTime");
            if calls.fetch_add(1, Ordering::SeqCst) < 2 {
                miss.clone()
            } else {
                json!({"result": 1_800_000_000})
            }
        });
        let entries = super::super::history::entries(&reader, &scan).unwrap();
        assert_eq!(entries[0].block_time, None);
        assert_eq!(entries[0].amount, 20);
        assert_eq!(entries[0].recipients, vec![recipient.owner.to_string()]);
        // A miss is remembered: the next read does not ask again.
        let entries = super::super::history::entries(&reader, &scan).unwrap();
        assert_eq!(entries[0].block_time, None);
        assert_eq!(asked.lock().unwrap().len(), 1);
        let wait_after = |reader: &MobileWallet, misses: u32| {
            let mut times = reader.block_times();
            let Some(BlockTime::Missing { retry_at, wait }) = times.get_mut(&10) else {
                panic!("the miss is cached");
            };
            assert_eq!(
                *wait,
                Duration::from_secs(30) * misses,
                "after {misses} misses"
            );
            *retry_at = Instant::now();
        };
        // Once the wait is over, it asks again; another miss doubles the wait.
        wait_after(&reader, 1);
        let entries = super::super::history::entries(&reader, &scan).unwrap();
        assert_eq!(entries[0].block_time, None);
        assert_eq!(asked.lock().unwrap().len(), 2);
        wait_after(&reader, 2);
        let entries = super::super::history::entries(&reader, &scan).unwrap();
        assert_eq!(entries[0].block_time, Some(1_800_000_000));
        assert_eq!(asked.lock().unwrap().len(), 3);
    }
}

#[test]
fn activity_time_reads_are_bounded_per_call_newest_first() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let fixture = Fixture::spend(&owner, &recipient, 20, 0, &[Mint::SOL]);
    let mut scan = fixture.scan(&owner);
    let first = scan.history.transactions[0].clone();
    scan.history.transactions = (1..=40)
        .map(|slot| {
            let mut tx = first.clone();
            tx.slot = slot;
            tx.tx_signature = Signature::from([slot as u8; 64]);
            tx
        })
        .collect();
    let (reader, asked) = wallet(&signer, |method, params| {
        assert_eq!(method, "getBlockTime");
        json!({"result": 1_800_000_000 + params[0].as_i64().unwrap()})
    });
    let entries = super::super::history::entries(&reader, &scan).unwrap();
    assert_eq!(entries.len(), 40);
    for entry in &entries {
        let expected = (entry.slot > 8).then_some(1_800_000_000 + entry.slot as i64);
        assert_eq!(entry.block_time, expected, "slot {}", entry.slot);
    }
    assert_eq!(asked.lock().unwrap().len(), 32, "newest slots first");
    let entries = super::super::history::entries(&reader, &scan).unwrap();
    assert!(entries.iter().all(|entry| entry.block_time.is_some()));
    assert_eq!(asked.lock().unwrap().len(), 40, "the rest on a later read");
}

#[test]
fn lookup_recipients_are_signature_wide_like_activity_for_mixed_asset_spends() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let public = identity(&Keypair::new());
    let token = Mint::new(Pubkey::new_unique(), 7);
    // A private token payment with a public SOL withdrawal in one spend.
    let fixture = Fixture::spend_legs(
        &owner,
        &[
            (Mint::SOL, &[], &[(&public, 30)]),
            (token, &[(&recipient, 20)], &[]),
        ],
        false,
    );
    let scan = fixture.scan(&owner);
    let mut expected = activity::entries(&scan.history, &scan.asset_registry).unwrap();
    assert_eq!(expected.len(), 2);
    for entry in &mut expected {
        entry.block_time = Some(0);
        assert_eq!(entry.recipients, vec![recipient.owner.to_string()]);
    }
    let signature = fixture.signature.to_string();
    let (reader, _) = fixture.open(&signer);
    let result = reader.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Ready);
    assert_eq!(result.entries, expected, "the same rows from every read");
    assert_eq!(result.outputs.len(), 2);
}

#[test]
fn an_inconsistent_lookup_response_seeds_no_block_time() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let mut fixture = Fixture::spend(&owner, &recipient, 20, 0, &[Mint::SOL]);
    fixture.events[0].slot = 11;
    let signature = fixture.signature.to_string();
    let (reader, _) = fixture.open(&signer);
    assert!(reader.transaction(signature).is_err());
    assert!(reader.block_times().is_empty());
}

#[test]
fn activity_recipient_tags_exclude_own_notes_and_deduplicate_across_events() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let collector = identity(&Keypair::new());
    let mut fixture = Fixture::spend_to_outputs(
        &owner,
        &[(&recipient, 20), (&collector, 5), (&recipient, 3)],
        &[],
        &[Mint::SOL],
        false,
    );
    fixture.events.push(fixture.events[0].clone());
    let scan = fixture.scan(&owner);
    let entries = activity::entries(&scan.history, &scan.asset_registry).unwrap();
    let mut expected = vec![recipient.owner.to_string(), collector.owner.to_string()];
    expected.sort();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].recipients, expected);
    assert_eq!(entries[0].amount, 28);
    // Do not present a ring's opaque discovery tags as Solana recipients.
    let mut history = scan.history;
    for tx in &mut history.transactions {
        tx.ring_config = Some(Pubkey::new_unique());
    }
    let entries = activity::entries(&history, &scan.asset_registry).unwrap();
    assert!(entries[0].recipients.is_empty());
}

#[test]
fn incoming_unrelated_and_self_transfers_are_read_without_history() {
    let sender = Keypair::new();
    let recipient = Keypair::new();
    let owner = identity(&sender);
    let other = identity(&recipient);
    let fixture = Fixture::spend(&owner, &other, 20, 0, &[Mint::SOL]);
    let signature = fixture.signature.to_string();
    let (wallet, _) = fixture.open(&recipient);
    let incoming = wallet.transaction(signature).unwrap();
    assert_eq!(incoming.status, Status::Ready);
    assert_eq!(incoming.entries.len(), 1);
    assert!(incoming.outputs.is_empty());
    assert!(incoming.entries[0].recipients.is_empty());
    assert_eq!(
        (incoming.entries[0].kind, incoming.entries[0].amount),
        (ActivityKind::Received, 20)
    );

    let fixture = Fixture::spend(&owner, &other, 20, 0, &[Mint::SOL]);
    let signature = fixture.signature.to_string();
    let (wallet, _) = fixture.open(&Keypair::new());
    let unrelated = wallet.transaction(signature).unwrap();
    assert_eq!(unrelated.status, Status::Ready);
    assert!(unrelated.entries.is_empty());
    assert!(unrelated.outputs.is_empty());

    let fixture = Fixture::spend(&owner, &owner, 20, 0, &[Mint::SOL]);
    let signature = fixture.signature.to_string();
    let (wallet, _) = fixture.open(&sender);
    let own = wallet.transaction(signature).unwrap();
    assert_eq!(own.status, Status::Ready);
    assert!(own.outputs.is_empty());
    assert!(own.entries[0].recipients.is_empty());
    assert_eq!(
        (own.entries[0].kind, own.entries[0].amount),
        (ActivityKind::SelfTransfer, 100)
    );
}

#[test]
fn indexing_delay_not_found_and_indexed_empty_are_distinct() {
    let signer = Keypair::new();
    for (chain_slot, indexed_slot, expected) in [
        (Some(10), 9, Status::PendingIndexing),
        (Some(10), 10, Status::Ready),
        (None, 10, Status::NotFound),
    ] {
        let owner = identity(&signer);
        let mut fixture = Fixture::spend(&owner, &owner, 20, 0, &[Mint::SOL]);
        fixture.events.clear();
        let indexed = fixture.indexer(indexed_slot);
        let chain = chain_slot.map(|_| fixture.chain).unwrap_or(Value::Null);
        let (wallet, asked) = wallet(&signer, move |method, _| match method {
            "getShieldedTransactionsBySignature" => json!({"result": indexed}),
            "getTransaction" => json!({"result": chain}),
            _ => panic!("unexpected request: {method}"),
        });
        let result = wallet
            .transaction(Signature::from([12; 64]).to_string())
            .unwrap();
        assert_eq!(result.status, expected);
        assert_eq!(result.slot, chain_slot);
        assert_eq!(result.block_time, chain_slot.map(|_| 0));
        assert_eq!(result.fee, chain_slot.map(|_| 5000));
        assert!(result.entries.is_empty());
        assert!(result.outputs.is_empty());
        assert_eq!(asked.lock().unwrap().len(), 2);
    }
}

#[test]
fn bad_signatures_and_network_failures_are_not_empty_results() {
    let signer = Keypair::new();
    let (wallet, asked) = wallet(
        &signer,
        |_, _| json!({"error": {"code": -32603, "message": "unavailable"}}),
    );
    assert_eq!(
        wallet.transaction("not-a-signature".into()),
        Err(WalletError::SignatureInvalid)
    );
    assert!(asked.lock().unwrap().is_empty());
    assert!(matches!(
        wallet.transaction(Signature::from([12; 64]).to_string()),
        Err(WalletError::Client { .. })
    ));
}

#[test]
fn missing_metadata_and_corrupt_ciphertexts_never_return_partial_rows() {
    let sender = Keypair::new();
    let recipient = Keypair::new();
    let owner = identity(&sender);
    let other = identity(&recipient);
    for mutation in [0, 1, 2] {
        let mut fixture = Fixture::spend(&owner, &other, 20, 0, &[Mint::SOL]);
        match mutation {
            0 => fixture.chain = Value::Null,
            1 => fixture.events[0].output_slots[0].payload[10] ^= 1,
            _ => fixture.records.clear(),
        }
        let signature = fixture.signature.to_string();
        let (wallet, _) = fixture.open(&sender);
        let result = wallet.transaction(signature).unwrap();
        assert_eq!(result.status, Status::Incomplete, "mutation {mutation}");
        assert!(result.entries.is_empty());
        assert!(result.outputs.is_empty());
        assert_eq!(result.fee, if mutation == 0 { None } else { Some(5000) });
    }
    // A recipient can still authenticate its received notes when the chain
    // RPC has pruned the metadata required for sender-side settlements.
    let mut fixture = Fixture::spend(&owner, &other, 20, 0, &[Mint::SOL]);
    fixture.chain = Value::Null;
    let signature = fixture.signature.to_string();
    let (wallet, _) = fixture.open(&recipient);
    let result = wallet.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Ready);
    assert_eq!(result.entries[0].amount, 20);
}

#[test]
fn missing_times_and_fees_are_optional_and_zero_fees_are_preserved() {
    let sender = Keypair::new();
    let receiver = Keypair::new();
    let owner = identity(&sender);
    let other = identity(&receiver);
    for fee in [None, Some(0)] {
        let mut fixture = Fixture::spend(&owner, &other, 20, 0, &[Mint::SOL]);
        fixture.chain["blockTime"] = Value::Null;
        if let Some(fee) = fee {
            fixture.chain["meta"]["fee"] = json!(fee);
        } else {
            fixture.chain["meta"] = Value::Null;
        }
        let signature = fixture.signature.to_string();
        let (reader, _) = fixture.open(&receiver);
        let result = reader.transaction(signature).unwrap();
        assert_eq!(result.status, Status::Ready);
        assert_eq!(result.block_time, None);
        assert_eq!(result.fee, fee);
        assert_eq!(result.entries[0].block_time, None);
        assert_eq!(result.entries[0].amount, 20);
    }
}

#[test]
fn an_invalid_later_output_discards_all_payment_details() {
    let signer = Keypair::new();
    let owner = identity(&signer);
    let recipient = identity(&Keypair::new());
    let collector = identity(&Keypair::new());
    let mut fixture = Fixture::spend_to_outputs(
        &owner,
        &[(&recipient, 20), (&collector, 5)],
        &[(&recipient, 10)],
        &[Mint::SOL],
        false,
    );
    fixture
        .records
        .get_mut(&collector.owner)
        .unwrap()
        .nullifier_pubkey[0] ^= 1;
    let signature = fixture.signature.to_string();
    let (reader, _) = fixture.open(&signer);
    let result = reader.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Incomplete);
    assert!(result.entries.is_empty());
    assert!(result.outputs.is_empty());
    assert_eq!(result.fee, Some(5000));
    assert_eq!(result.block_time, Some(0));
}

#[test]
fn deposit_events_are_aggregated_deduplicated_and_have_stable_asset_ids() {
    use zolana_transaction::serialization::{
        OwnerCx, Proofless, ProoflessEncode, UtxoSerialization,
    };
    let signer = Keypair::new();
    let owner = identity(&signer);
    let token = Mint::new(Pubkey::new_unique(), 7);
    let mut fixture = Fixture::spend(&owner, &owner, 20, 0, &[Mint::SOL, token]);
    let tag = owner
        .address
        .signing_pubkey
        .confidential_view_tag()
        .unwrap();
    let mut slots = Vec::new();
    for (index, asset) in [Mint::SOL, token].iter().enumerate() {
        let utxo = Utxo {
            owner: owner.address.signing_pubkey,
            asset: *asset,
            amount: 5 + index as u64,
            blinding: [index as u8; 32],
            ring_program_id: None,
            data: Default::default(),
        };
        let hash = utxo
            .hash(&owner.address.nullifier_pubkey, &[0; 32], &[0; 32], 0)
            .unwrap();
        let encoded = Proofless::encode(
            &[utxo],
            &OwnerCx {
                owner: owner.address.signing_pubkey,
                assets: &fixture.registry,
                ring_program_id: None,
                first_nullifier: None,
            },
            tag,
            &ProoflessEncode {
                owner_hash: owner.address.owner_hash().unwrap(),
                data_hash: None,
                ring_data_hash: None,
            },
        )
        .unwrap();
        slots.push(OutputSlot {
            view_tag: tag,
            output_context: OutputContext {
                hash,
                tree_id: 0,
                leaf_index: 100 + index as u64,
            },
            payload: encoded.data,
        });
    }
    fixture.events[0].output_slots = slots;
    fixture.events[0].proofless = true;
    fixture.events[0].tx_viewing_pk = None;
    fixture.events[0].salt = None;
    fixture.events[0].nullifiers.clear();
    fixture.chain = Value::Null;
    let mut duplicate = fixture.events[0].clone();
    duplicate.event_index = Some(1);
    fixture.events.push(duplicate);
    let signature = fixture.signature.to_string();
    let (wallet, _) = fixture.open(&signer);
    let result = wallet.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Ready);
    assert_eq!(result.entries.len(), 2);
    assert_eq!(
        result
            .entries
            .iter()
            .map(|entry| (entry.index, entry.amount))
            .collect::<Vec<_>>(),
        vec![(1, 5), (7, 6)]
    );
    assert!(result
        .entries
        .iter()
        .all(|entry| entry.kind == ActivityKind::Deposit));
}

#[test]
fn a_cold_wallet_reads_a_merge_as_a_self_transfer() {
    use zolana_interface::event::{encode_merge_output_data, MergeOutputDataRef};
    use zolana_transaction::instructions::merge::{
        merge_output_blinding, merge_output_key, MergeOutputPlaintext,
    };
    let signer = Keypair::new();
    let owner = identity(&signer);
    let mut fixture = Fixture::spend(&owner, &owner, 20, 0, &[Mint::SOL]);
    let first = [1; 32];
    let salt = [3; 16];
    let key = merge_output_key(&owner.nullifier_key, &first).unwrap();
    let blinding = merge_output_blinding(&owner.nullifier_key, &first).unwrap();
    let plaintext = MergeOutputPlaintext {
        amount: 50,
        mint: SOL_MINT,
    };
    let ciphertext = plaintext.encrypt(&key, &salt).unwrap();
    let utxo = plaintext.into_utxo(owner.address.signing_pubkey, Mint::SOL, blinding, None);
    let hash = utxo
        .hash(&owner.address.nullifier_pubkey, &[0; 32], &[0; 32], 0)
        .unwrap();
    fixture.events[0].output_slots = vec![OutputSlot {
        view_tag: owner
            .address
            .signing_pubkey
            .confidential_view_tag()
            .unwrap(),
        output_context: OutputContext {
            hash,
            tree_id: 0,
            leaf_index: 100,
        },
        payload: encode_merge_output_data(MergeOutputDataRef {
            salt: &salt,
            ciphertext: &ciphertext,
            prefix: &[],
        }),
    }];
    fixture.events[0].tx_viewing_pk = None;
    fixture.events[0].salt = None;
    fixture.events[0].nullifiers = vec![first];
    fixture.chain = Value::Null;
    let mut corrupt = Fixture {
        signature: fixture.signature,
        chain: fixture.chain.clone(),
        events: fixture.events.clone(),
        registry: fixture.registry.clone(),
        notes: fixture.notes.clone(),
        records: fixture.records.clone(),
        openings: fixture.openings.clone(),
    };
    corrupt.events[0].output_slots[0].output_context.hash = [0; 32];
    let signature = fixture.signature.to_string();
    let (wallet, _) = fixture.open(&signer);
    let result = wallet.transaction(signature.clone()).unwrap();
    assert_eq!(result.status, Status::Ready);
    assert_eq!(
        (result.entries[0].kind, result.entries[0].amount),
        (ActivityKind::SelfTransfer, 50)
    );
    let (wallet, _) = corrupt.open(&signer);
    let result = wallet.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Incomplete);
    assert!(result.entries.is_empty());
}

#[test]
fn a_full_private_send_with_recipient_tagged_padding_is_explicitly_incomplete() {
    let sender = Keypair::new();
    let recipient = Keypair::new();
    let fixture = Fixture::spend(
        &identity(&sender),
        &identity(&recipient),
        100,
        0,
        &[Mint::SOL],
    );
    let signature = fixture.signature.to_string();
    let (wallet, _) = fixture.open(&sender);
    let result = wallet.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Incomplete);
    assert!(result.entries.is_empty());
}

#[test]
fn padding_openings_resolve_exact_balance_payments_without_history() {
    let sender = Keypair::new();
    let recipient = Keypair::new();
    for cached in [false, true] {
        for (private, withdrawal) in [(100, 0), (50, 50)] {
            for mint in [Mint::SOL, Mint::new(Pubkey::new_unique(), 7)] {
                let fixture = Fixture::spend_with_cache(
                    &identity(&sender),
                    &identity(&recipient),
                    private,
                    withdrawal,
                    &[mint],
                    cached,
                );
                let signature = fixture.signature.to_string();
                let pending = fixture.prepared(&sender);
                let (wallet, _) = fixture.open(&sender);
                let result = wallet
                    .transaction_with_prepared(signature.clone(), &pending)
                    .unwrap();
                assert_eq!(result.status, Status::Ready);
                assert_eq!(result.entries.len(), 1);
                let entry = &result.entries[0];
                assert_eq!(entry.kind, ActivityKind::Sent);
                assert_eq!(entry.amount, 100);
                assert_eq!(entry.mint, crate::asset::mint_name(&mint.asset));
                assert_eq!(entry.signature, signature);
                assert_eq!(entry.slot, 10);
                assert_eq!(
                    result
                        .outputs
                        .iter()
                        .map(|output| output.amount)
                        .sum::<u64>(),
                    100
                );
            }
        }
    }
}

#[test]
fn padding_openings_cannot_hide_missing_or_tampered_details() {
    let sender = Keypair::new();
    let recipient = Keypair::new();
    for mutation in 0..6 {
        let mut fixture = Fixture::spend(
            &identity(&sender),
            &identity(&recipient),
            100,
            0,
            &[Mint::SOL],
        );
        let mut pending = fixture.prepared(&sender);
        let dummy = pending
            .outputs
            .iter()
            .position(|output| output.owner_signing_key.is_none() && output.amount == 0)
            .unwrap();
        match mutation {
            0 => pending.outputs[dummy].blinding[31] ^= 1,
            1 => {
                pending = Fixture::spend(
                    &identity(&sender),
                    &identity(&recipient),
                    100,
                    0,
                    &[Mint::SOL],
                )
                .prepared(&sender)
            }
            2 => fixture.chain = Value::Null,
            3 => fixture.records.clear(),
            4 => {
                // A real output cannot open under the dummy commitment domain.
                let real = pending
                    .outputs
                    .iter()
                    .find(|output| output.owner_signing_key.is_some())
                    .unwrap()
                    .blinding
                    .clone();
                let mut false_padding = pending.outputs[dummy].clone();
                false_padding.blinding = real;
                pending.outputs.push(false_padding);
                fixture
                    .records
                    .values_mut()
                    .next()
                    .unwrap()
                    .nullifier_pubkey = [0; 32];
            }
            _ => fixture.events[0].output_slots[0].payload[10] ^= 1,
        }
        let signature = fixture.signature.to_string();
        let (wallet, _) = fixture.open(&sender);
        let result = wallet
            .transaction_with_prepared(signature, &pending)
            .unwrap();
        assert_eq!(result.status, Status::Incomplete, "mutation {mutation}");
        assert!(result.entries.is_empty());
    }
}

#[test]
fn a_padding_opening_for_another_tree_cannot_hide_an_unreadable_slot() {
    let sender = Keypair::new();
    let recipient = Keypair::new();
    let fixture = Fixture::spend(
        &identity(&sender),
        &identity(&recipient),
        100,
        0,
        &[Mint::SOL],
    );
    let signature = fixture.signature.to_string();
    let mut pending = fixture.prepared(&sender);
    for opening in &mut pending.outputs {
        opening.tree_id += 1;
    }
    let (wallet, _) = fixture.open(&sender);
    let result = wallet
        .transaction_with_prepared(signature, &pending)
        .unwrap();
    assert_eq!(result.status, Status::Incomplete);
    assert!(result.entries.is_empty());
}

#[test]
fn a_cold_token_lookup_discovers_only_asset_metadata_and_keeps_existing_mappings() {
    let sender = Keypair::new();
    let receiver = Keypair::new();
    let token = Mint::new(Pubkey::new_unique(), 7);
    let fixture = Fixture::spend(
        &identity(&sender),
        &identity(&receiver),
        10,
        20,
        &[Mint::SOL, token],
    );
    let signature = fixture.signature.to_string();
    let (wallet, asked) = fixture.open_with_registry(&sender, false);
    let result = wallet.transaction(signature).unwrap();
    assert_eq!(result.status, Status::Ready);
    assert_eq!(
        result
            .entries
            .iter()
            .map(|row| (row.index, row.amount))
            .collect::<Vec<_>>(),
        vec![(1, 30), (7, 30)]
    );
    assert_eq!(
        asked
            .lock()
            .unwrap()
            .iter()
            .filter(|method| method.as_str() == "getProgramAccounts")
            .count(),
        1
    );
}
