//! Signature-scoped activity, using the protocol's existing readers. No
//! transaction or nullifier history queries belong on this path.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    str::FromStr,
};

use solana_pubkey::Pubkey;
use solana_rpc_client_api::request::RpcRequest;
use solana_signature::Signature;
use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;
use spr_client::{
    fetch_spl_asset_registrations, rpc::solana_rpc::ConfirmedInstructionGroups, ClientError, Rpc,
};
use spr_interface::{
    event::OutputDataEncoding,
    instruction::{fetch_tag, settlement_accounts, tag, CircuitId, TransactIxData},
    PROGRAM_ID_PUBKEY,
};
use spr_keypair::{KeypairError, PublicKey, ViewingKey};
use spr_transaction::{
    decrypt, instructions::transact::SppProofOutputUtxo, serialization::confidential::Confidential,
    verify_owned, AssetRegistry, EncryptedScheme, HistoryEntry, HistoryKind, OutputSlot,
    ShieldedKeys, ShieldedTransaction, TransactionError, WalletUtxo, SOL_MINT,
};
use spr_user_registry_interface::UserRecord;

use super::{
    history,
    prepared::PreparedTransaction,
    types::OutputOpening,
    util::{beside, invalid},
    MobileWallet,
};
use crate::{
    activity::{
        self, ActivityEntry, TransactionLookup, TransactionLookupStatus as Status,
        TransactionOutput,
    },
    asset::mint_name,
    error::WalletError,
};

type Confirmed = EncodedConfirmedTransactionWithStatusMeta;

pub(super) fn transaction(
    wallet: &MobileWallet,
    signature: String,
    prepared: Option<&PreparedTransaction>,
) -> Result<TransactionLookup, WalletError> {
    let signature = Signature::from_str(&signature).map_err(|_| WalletError::SignatureInvalid)?;
    let padding = padding_commitments(prepared.map_or(&[], |pending| pending.outputs.as_slice()))?;
    // A single confirmed RPC read, including a real `null` when unavailable.
    // The convenience transaction fetcher polls missing signatures for 30s.
    let (indexed, confirmed) = beside(
        || {
            wallet
                .client
                .get_shielded_transactions_by_signature(signature, None)
        },
        || {
            wallet.client.rpc().client().send::<Option<Confirmed>>(
                RpcRequest::GetTransaction,
                serde_json::json!([signature.to_string(), {
                    "encoding": "json", "commitment": "confirmed", "maxSupportedTransactionVersion": 1
                }]),
            )
        },
    );
    let indexed = indexed?;
    let confirmed = confirmed.map_err(|error| ClientError::Rpc(error.to_string()))?;
    let mut event_ids = HashSet::new();
    let mut transactions = Vec::with_capacity(indexed.transactions.len());
    let slot = confirmed
        .as_ref()
        .map(|tx| tx.slot)
        .or_else(|| indexed.transactions.first().map(|tx| tx.transaction.slot));
    let block_time = confirmed.as_ref().and_then(|tx| tx.block_time);
    let fee = confirmed
        .as_ref()
        .and_then(|tx| tx.transaction.meta.as_ref())
        .map(|meta| meta.fee);
    for event in indexed.transactions {
        let tx = event.transaction;
        if tx.tx_signature != signature
            || Some(tx.slot) != slot
            || !event_ids.insert(event.event_index)
        {
            return Err(ClientError::Rpc("inconsistent signature lookup response".into()).into());
        }
        transactions.push(tx);
    }
    // Seed the shared cache only with a slot the indexed events agree on.
    if let (Some(slot), Some(time)) = (slot, block_time) {
        wallet
            .block_times()
            .insert(slot, history::BlockTime::Known(time));
    }
    let result = |status, mut entries: Vec<ActivityEntry>, outputs: Vec<TransactionOutput>| {
        for entry in &mut entries {
            entry.block_time = block_time;
        }
        TransactionLookup {
            status,
            entries,
            outputs,
            slot,
            block_time,
            fee,
            indexed_slot: indexed.context.slot,
        }
    };
    if slot.is_none() {
        return Ok(result(Status::NotFound, vec![], vec![]));
    }
    if slot.is_some_and(|slot| slot > indexed.context.slot) {
        return Ok(result(Status::PendingIndexing, vec![], vec![]));
    }
    if transactions.is_empty() {
        return Ok(result(Status::Ready, vec![], vec![]));
    }
    if confirmed
        .as_ref()
        .and_then(|tx| tx.transaction.meta.as_ref())
        .is_some_and(|meta| meta.err.is_some())
    {
        return Err(ClientError::Rpc(
            "indexer returned shielded events for a failed transaction".into(),
        )
        .into());
    }
    let (discover, mut registry) = {
        let mut assets = wallet.assets();
        (assets.discovers(), assets.registry(&wallet.client)?.clone())
    };
    let mut details = wallet.lookup_details(
        signature,
        &transactions,
        &registry,
        discover,
        confirmed.as_ref(),
        &padding,
    );
    if matches!(details, Err(Halt::NeedAssets)) && discover {
        for mint in fetch_spl_asset_registrations(&wallet.client, slot)? {
            if registry.asset_id(&mint.asset).ok() != Some(mint.asset_id) {
                registry.insert(mint.asset_id, mint.asset)?;
            }
        }
        details = wallet.lookup_details(
            signature,
            &transactions,
            &registry,
            false,
            confirmed.as_ref(),
            &padding,
        );
    }
    match details {
        Ok((entries, outputs)) => {
            wallet.assets().remember_registry(registry);
            Ok(result(Status::Ready, entries, outputs))
        }
        Err(Halt::Incomplete | Halt::NeedAssets) => Ok(result(Status::Incomplete, vec![], vec![])),
        Err(Halt::Failed(failure)) => Err(failure),
    }
}

/// The commitments of the dummy outputs among a prepared spend's openings,
/// with their trees. Each is recomputed from its blinding, so an opening that
/// does not hash to a published commitment matches nothing.
fn padding_commitments(outputs: &[OutputOpening]) -> Result<HashSet<(u16, [u8; 32])>, WalletError> {
    outputs
        .iter()
        .filter(|output| {
            output.owner_signing_key.is_none()
                && output.nullifier_key.is_none()
                && output.amount == 0
                && !output.compact
        })
        .map(|output| {
            let blinding = output
                .blinding
                .as_slice()
                .try_into()
                .map_err(|_| invalid("invalid output blinding"))?;
            let dummy = SppProofOutputUtxo {
                blinding,
                ..Default::default()
            };
            Ok((output.tree_id, dummy.hash(output.tree_id)?))
        })
        .collect()
}

/// Why reading a lookup's rows stopped: assets to discover before a second
/// read, a transaction this reader cannot read, or a failure.
enum Halt {
    NeedAssets,
    Incomplete,
    Failed(WalletError),
}

impl From<WalletError> for Halt {
    fn from(failure: WalletError) -> Self {
        Self::Failed(failure)
    }
}

impl From<ClientError> for Halt {
    fn from(failure: ClientError) -> Self {
        Self::Failed(failure.into())
    }
}

impl From<TransactionError> for Halt {
    fn from(failure: TransactionError) -> Self {
        Self::Failed(failure.into())
    }
}

impl From<KeypairError> for Halt {
    fn from(failure: KeypairError) -> Self {
        Self::Failed(failure.into())
    }
}

#[derive(Default)]
struct Movement {
    received: u128,
    spent: u128,
}

/// What the transactions of one signature move for this wallet, as they are
/// read.
#[derive(Default)]
struct Tally {
    movements: BTreeMap<Pubkey, Movement>,
    outputs: Vec<TransactionOutput>,
    /// The wallet's notes counted as received.
    counted: HashSet<[u8; 32]>,
    /// The spends counted, by their nullifiers.
    spends: HashSet<Vec<[u8; 32]>>,
    /// An output goes to another wallet.
    pays_another: bool,
}

/// What reading one signature's transactions needs of the wallet.
struct Reader<'a> {
    wallet: &'a MobileWallet,
    assets: &'a AssetRegistry,
    /// An unknown asset asks for a second read after discovery.
    discover: bool,
    /// The wallet discovers assets, so an unknown one leaves the lookup
    /// incomplete rather than out of its rows.
    discovers: bool,
    padding: &'a HashSet<(u16, [u8; 32])>,
    /// The wallet's notes among the outputs, by commitment.
    owned: HashMap<[u8; 32], &'a WalletUtxo>,
    own_tag: [u8; 32],
    own_hash: [u8; 32],
    /// Every view tag of the wallet, `own_tag` included.
    tags: HashSet<[u8; 32]>,
    /// The confirmed transaction's instructions.
    groups: Option<ConfirmedInstructionGroups>,
    /// The decrypted outputs named no unknown mint.
    mints_known: bool,
}

impl MobileWallet {
    /// This wallet's rows of `transactions`, the events of `signature`, and
    /// the payments they make.
    fn lookup_details(
        &self,
        signature: Signature,
        transactions: &[ShieldedTransaction],
        assets: &AssetRegistry,
        discover: bool,
        confirmed: Option<&Confirmed>,
        padding: &HashSet<(u16, [u8; 32])>,
    ) -> Result<(Vec<ActivityEntry>, Vec<TransactionOutput>), Halt> {
        let decoded = decrypt(&self.keys, transactions, assets)?;
        let unknown = !decoded.unknown_asset_ids.is_empty() || !decoded.unknown_mints.is_empty();
        let discovers = self.assets().discovers();
        if unknown && discover {
            return Err(Halt::NeedAssets);
        }
        if unknown && discovers {
            return Err(Halt::Incomplete);
        }
        let owned = verify_owned(&self.keys, &decoded)?;
        let own_tag = self.address.signing_pubkey.confidential_view_tag()?;
        let mut tags: HashSet<_> = self
            .keys
            .viewing_public_keys()
            .iter()
            .map(|key| key.u())
            .collect();
        tags.insert(own_tag);
        let reader = Reader {
            wallet: self,
            assets,
            discover,
            discovers,
            padding,
            owned: owned
                .utxos
                .iter()
                .map(|utxo| (utxo.utxo_hash, utxo))
                .collect(),
            own_tag,
            own_hash: self.address.owner_hash()?,
            tags,
            groups: confirmed
                .cloned()
                .and_then(|tx| ConfirmedInstructionGroups::try_from(tx).ok()),
            mints_known: decoded.unknown_mints.is_empty(),
        };
        let mut tally = Tally::default();
        for tx in transactions {
            reader.read(tx, &mut tally)?;
        }
        let entries = reader.entries(signature, transactions, &tally)?;
        Ok((entries, tally.outputs))
    }
}

impl Reader<'_> {
    /// Count `tx` into `tally`.
    fn read(&self, tx: &ShieldedTransaction, tally: &mut Tally) -> Result<(), Halt> {
        let received = self.receive(tx, tally)?;
        if tx.proofless {
            return Ok(());
        }
        if is_merge(tx) {
            return self.merge(tx, received, tally);
        }
        self.spend(tx, received, tally)
    }

    /// What `tx` pays this wallet, per mint, each note counted once into
    /// `tally`. An output for this wallet that it cannot read stops the read.
    fn receive(
        &self,
        tx: &ShieldedTransaction,
        tally: &mut Tally,
    ) -> Result<BTreeMap<Pubkey, u128>, Halt> {
        let mut received = BTreeMap::<Pubkey, u128>::new();
        for slot in &tx.output_slots {
            if let Some(note) = self.owned.get(&slot.output_context.hash) {
                *received.entry(note.utxo.asset.asset).or_default() += u128::from(note.utxo.amount);
                if tally.counted.insert(note.utxo_hash) {
                    tally
                        .movements
                        .entry(note.utxo.asset.asset)
                        .or_default()
                        .received += u128::from(note.utxo.amount);
                }
            } else if self.unread_own(slot) {
                return Err(Halt::Incomplete);
            } else if !self.tags.contains(&slot.view_tag) {
                tally.pays_another = true;
            }
        }
        Ok(received)
    }

    /// Whether `slot` is for this wallet although its notes are not: an
    /// output encrypted to its viewing key, or a deposit to it of a known
    /// asset.
    fn unread_own(&self, slot: &OutputSlot) -> bool {
        matches!(slot.output_data(), Some(OutputDataEncoding::Encrypted(ref blob))
            if blob.first() == Some(&EncryptedScheme::Confidential.as_byte()) &&
            Confidential::embedded_viewing_pk(&blob[1..]).ok()
                == Some(self.wallet.address.viewing_pubkey))
            || slot.proofless_output().is_some_and(|output| {
                output.owner == self.own_hash
                    && self
                        .assets
                        .asset_id(&Pubkey::new_from_array(output.asset))
                        .is_ok()
            })
    }

    /// A merge spends what it pays the wallet. The existing merge decoder
    /// derives the key from the owner's nullifier secret and accepts only
    /// commitment-matching notes.
    fn merge(
        &self,
        tx: &ShieldedTransaction,
        received: BTreeMap<Pubkey, u128>,
        tally: &mut Tally,
    ) -> Result<(), Halt> {
        if received.is_empty()
            && self.mints_known
            && tx
                .output_slots
                .iter()
                .any(|slot| self.tags.contains(&slot.view_tag))
        {
            return Err(Halt::Incomplete);
        }
        if !received.is_empty() && tally.spends.insert(tx.nullifiers.clone()) {
            for (mint, amount) in received {
                tally.movements.entry(mint).or_default().spent += amount;
            }
        }
        Ok(())
    }

    /// A spend of this wallet's notes, read with its transaction viewing key
    /// against the confirmed instruction. A spend of another wallet's notes
    /// counts nothing.
    fn spend(
        &self,
        tx: &ShieldedTransaction,
        received: BTreeMap<Pubkey, u128>,
        tally: &mut Tally,
    ) -> Result<(), Halt> {
        let Some(first) = tx.nullifiers.first() else {
            return Ok(());
        };
        let key = self.wallet.viewing_key.get_transaction_viewing_key(first)?;
        let context = self
            .groups
            .as_ref()
            .and_then(|groups| transact_context(tx, groups));
        if self.groups.is_some() && tx.ring_config.is_none() && context.is_none() {
            return Err(Halt::Incomplete);
        }
        if Some(key.pubkey()) != tx.tx_viewing_pk {
            // A sponsor signing as fee payer owns no inputs merely by
            // paying. An actual owner whose key cannot read the spend
            // needs context this bounded reader does not have.
            if context
                .as_ref()
                .is_some_and(|cx| cx.owners.contains(&self.wallet.owner))
            {
                return Err(Halt::Incomplete);
            }
            return Ok(());
        }
        let Some(cx) = context else {
            return Err(Halt::Incomplete);
        };
        if !matches!(
            cx.data.circuit.uncached(),
            CircuitId::ConfidentialSigner(..)
        ) || cx.owners != [self.wallet.owner]
        {
            return Err(Halt::Incomplete);
        }
        if !tally.spends.insert(tx.nullifiers.clone()) {
            return Ok(());
        }
        // Outputs owned by somebody else and public settlement legs are
        // the outflow. Change cancels out, so no input-note history is
        // required. Self-transfers instead spend exactly their kept value.
        let mut net = BTreeMap::<Pubkey, i128>::new();
        self.private_outflow(tx, &key, &mut net, tally)?;
        self.public_outflow(&cx, &mut net, tally)?;
        for mint in received.keys() {
            net.entry(*mint).or_default();
        }
        for (mint, net) in net {
            let kept = received.get(&mint).copied().unwrap_or_default();
            let spent = kept as i128 + net;
            if spent < 0 {
                return Err(Halt::Incomplete);
            }
            tally.movements.entry(mint).or_default().spent += spent as u128;
        }
        Ok(())
    }

    /// The registry records of the wallets `tx` pays, by owner.
    fn recipients(&self, tx: &ShieldedTransaction) -> Result<HashMap<Pubkey, UserRecord>, Halt> {
        let owners: Vec<_> = tx
            .output_slots
            .iter()
            .filter(|slot| slot.view_tag != self.own_tag)
            .map(|slot| Pubkey::new_from_array(slot.view_tag))
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        if owners.is_empty() {
            return Ok(HashMap::new());
        }
        let response = self.wallet.client.get_user_records(owners.clone(), None)?;
        if response.records.len() != owners.len() {
            return Err(Halt::Incomplete);
        }
        let mut records = HashMap::new();
        for (owner, record) in owners.into_iter().zip(response.records) {
            match record {
                Some(record) if record.owner == owner => records.insert(owner, record),
                _ => return Err(Halt::Incomplete),
            };
        }
        Ok(records)
    }

    /// The private outputs of `tx` to other wallets, each decrypted with
    /// `key` and checked against its commitment, padding left out.
    fn private_outflow(
        &self,
        tx: &ShieldedTransaction,
        key: &ViewingKey,
        net: &mut BTreeMap<Pubkey, i128>,
        tally: &mut Tally,
    ) -> Result<(), Halt> {
        let records = self.recipients(tx)?;
        let Some(salt) = tx.salt else {
            return Err(Halt::Incomplete);
        };
        for (index, slot) in tx.output_slots.iter().enumerate() {
            if slot.view_tag == self.own_tag
                || self
                    .padding
                    .contains(&(slot.output_context.tree_id, slot.output_context.hash))
            {
                continue;
            }
            let Some(OutputDataEncoding::Encrypted(blob)) = slot.output_data() else {
                return Err(Halt::Incomplete);
            };
            let Some((&scheme, body)) = blob.split_first() else {
                return Err(Halt::Incomplete);
            };
            if scheme != EncryptedScheme::Confidential.as_byte() {
                return Err(Halt::Incomplete);
            }
            let Ok(plaintext) = Confidential::decrypt_with_tx_key(key, body, salt, index as u32)
            else {
                return Err(Halt::Incomplete);
            };
            let record = &records[&Pubkey::new_from_array(slot.view_tag)];
            let owner = PublicKey::from_ed25519(record.owner.as_array());
            let note = match plaintext.into_utxo(owner, self.assets) {
                Ok(note) => note,
                Err(TransactionError::UnknownAsset(_)) if self.discover => {
                    return Err(Halt::NeedAssets)
                }
                Err(TransactionError::UnknownAsset(_)) if !self.discovers => continue,
                Err(_) => return Err(Halt::Incomplete),
            };
            if note.ring_program_id.is_some()
                || note.data.utxo_data().is_some()
                || note.data.ring_data().is_some()
                || note.hash(
                    &record.nullifier_pubkey,
                    &[0; 32],
                    &[0; 32],
                    slot.output_context.tree_id,
                )? != slot.output_context.hash
            {
                return Err(Halt::Incomplete);
            }
            *net.entry(note.asset.asset).or_default() += i128::from(note.amount);
            tally.outputs.push(TransactionOutput {
                recipient: record.owner.to_string(),
                mint: mint_name(&note.asset.asset),
                amount: note.amount,
                is_public: false,
            });
        }
        Ok(())
    }

    /// The public settlement legs of the spend `cx`: a withdrawal flows out,
    /// a deposit in.
    fn public_outflow(
        &self,
        cx: &TransactContext,
        net: &mut BTreeMap<Pubkey, i128>,
        tally: &mut Tally,
    ) -> Result<(), Halt> {
        let Some(settlements) =
            settlement_accounts(&cx.data.interface_transfers, cx.data.circuit, &cx.accounts)
        else {
            return Err(Halt::Incomplete);
        };
        let mut offset = 0;
        for leg in &cx.data.interface_transfers {
            let mint = leg
                .mint_account_position()
                .map_or(SOL_MINT, |i| settlements[offset + i]);
            // SOL: [interface, recipient]. SPL withdrawal:
            // [cpi_authority, mint, interface, token_account, program].
            let recipient = settlements[offset + if leg.is_spl() { 3 } else { 1 }];
            offset += leg.settlement_account_count();
            if self.assets.asset_id(&mint).is_err() {
                if self.discover {
                    return Err(Halt::NeedAssets);
                }
                if self.discovers {
                    return Err(Halt::Incomplete);
                }
                continue;
            }
            *net.entry(mint).or_default() += if leg.is_deposit() {
                -i128::from(leg.amount())
            } else {
                i128::from(leg.amount())
            };
            if !leg.is_deposit() {
                tally.outputs.push(TransactionOutput {
                    recipient: recipient.to_string(),
                    mint: mint_name(&mint),
                    amount: leg.amount(),
                    is_public: true,
                });
            }
        }
        Ok(())
    }

    /// One row per mint `tally` moved.
    fn entries(
        &self,
        signature: Signature,
        transactions: &[ShieldedTransaction],
        tally: &Tally,
    ) -> Result<Vec<ActivityEntry>, Halt> {
        // The same signature-wide rule as activity and overview, so the
        // same row reads equal from every path.
        let recipients: Vec<_> = transactions
            .iter()
            .flat_map(|tx| {
                activity::recipients(tx, |slot| {
                    self.tags.contains(&slot.view_tag)
                        || self.owned.contains_key(&slot.output_context.hash)
                })
            })
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let deposit = transactions.iter().any(|tx| tx.proofless);
        let mut entries = Vec::new();
        for (&mint, movement) in &tally.movements {
            let (kind, amount) = movement_kind(movement, deposit, tally.pays_another);
            if amount == 0 {
                continue;
            }
            let amount = u64::try_from(amount)
                .map_err(|_| ClientError::Rpc("activity amount exceeds u64".into()))?;
            let mut entry = activity::activity_entry(
                HistoryEntry {
                    kind,
                    mint,
                    amount,
                    tx_signature: signature,
                    slot: transactions[0].slot,
                },
                self.assets,
            )?;
            if matches!(kind, HistoryKind::Sent | HistoryKind::Withdrawal) {
                entry.recipients = recipients.clone();
            }
            entries.push(entry);
        }
        Ok(entries)
    }
}

/// Whether `tx` may be a merge and carries a merge-encrypted output.
fn is_merge(tx: &ShieldedTransaction) -> bool {
    tx.may_be_merge()
        && tx.output_slots.iter().any(|slot| {
            matches!(slot.output_data(), Some(OutputDataEncoding::VerifiablyEncrypted(ref blob) | OutputDataEncoding::Encrypted(ref blob))
                if blob.first() == Some(&EncryptedScheme::Merge.as_byte()))
        })
}

/// The row a mint's movement makes, and its amount.
fn movement_kind(movement: &Movement, deposit: bool, pays_another: bool) -> (HistoryKind, u128) {
    let (received, spent) = (movement.received, movement.spent);
    if spent == 0 {
        let kind = if deposit {
            HistoryKind::Deposit
        } else {
            HistoryKind::Received
        };
        (kind, received)
    } else if spent == received {
        (HistoryKind::SelfTransfer, spent)
    } else if received > spent {
        (HistoryKind::Deposit, received - spent)
    } else {
        let kind = if pays_another {
            HistoryKind::Sent
        } else {
            HistoryKind::Withdrawal
        };
        (kind, spent - received)
    }
}

struct TransactContext {
    data: TransactIxData,
    accounts: Vec<Pubkey>,
    owners: Vec<Pubkey>,
}

/// Bind the indexer's event to a real SPP invocation, including ciphertexts,
/// commitments and resolved owner tags. A matching byte sequence elsewhere in
/// the transaction is not an instruction or evidence for a settlement.
fn transact_context(
    tx: &ShieldedTransaction,
    groups: &ConfirmedInstructionGroups,
) -> Option<TransactContext> {
    for instruction in groups
        .groups
        .iter()
        .flat_map(|group| std::iter::once(&group.outer).chain(&group.inner))
    {
        if instruction.program_id != PROGRAM_ID_PUBKEY
            || instruction.data.first() != Some(&tag::TRANSACT)
        {
            continue;
        }
        let data = TransactIxData::deserialize(instruction.data.get(1..)?).ok()?;
        if data
            .inputs
            .iter()
            .map(|input| input.nullifier_hash)
            .collect::<Vec<_>>()
            != tx.nullifiers
            || data.outputs.len() != tx.output_slots.len()
            || Some(data.tx_viewing_pk) != tx.tx_viewing_pk.map(|key| *key.as_bytes())
            || Some(data.salt) != tx.salt
        {
            continue;
        }
        let same_outputs = data
            .outputs
            .iter()
            .zip(&tx.output_slots)
            .all(|(output, slot)| {
                output.utxo_hash == slot.output_context.hash
                    && output.data.as_deref() == Some(slot.payload.as_slice())
                    && fetch_tag(&output.owner_tag, |i| {
                        instruction
                            .accounts
                            .get(usize::from(i))
                            .map(|key| key.to_bytes())
                    })
                    .ok()
                        == Some(slot.view_tag)
            });
        if !same_outputs {
            continue;
        }
        let settlements = settlement_accounts(
            &data.interface_transfers,
            data.circuit,
            &instruction.accounts,
        )?;
        let start = 4 + data.tree_contexts.len() + data.inputs.len();
        let cache_accounts = data.circuit.cache_access().map_or(0, |access| {
            usize::from(access.read_bitmap != 0) + 2 * usize::from(access.writes_cache())
        });
        let end = instruction
            .accounts
            .len()
            .checked_sub(cache_accounts + settlements.len())?;
        let owners = instruction.accounts.get(start..end)?.to_vec();
        return Some(TransactContext {
            data,
            accounts: instruction.accounts.clone(),
            owners,
        });
    }
    None
}

impl MobileWallet {
    /// Fetch only the shielded events of `signature` and their confirmed
    /// Solana transaction, then decrypt this wallet's rows. Does not scan or
    /// warm the wallet's history or spendable-note cache. Asset/recipient
    /// registry reads may be needed, but no history pages are fetched.
    ///
    /// A cold wallet can read its default-ring outgoing transfers and public
    /// withdrawals using its transaction viewing key, without old input notes.
    /// `outputs` keeps each private payment and public withdrawal separate,
    /// excluding own change and padding. Block time and network fee come from
    /// the same confirmed RPC read, including when indexing is still pending.
    /// `Incomplete` reports unsupported spends or missing metadata rather than
    /// returning an incorrect empty result. Recipient-tagged dummy padding
    /// (possible on an exact-balance private send) is also incomplete: a cold
    /// reader cannot authenticate it as padding without an output opening.
    /// Use [`Self::transaction_with_prepared`] when the prepared spend was
    /// saved. Only `Ready` with no entries means there are no activity rows
    /// for this wallet's configured assets.
    pub fn transaction(&self, signature: String) -> Result<TransactionLookup, WalletError> {
        transaction(self, signature, None)
    }

    /// Like [`Self::transaction`], with output openings from a prepared spend
    /// (including one restored after restart). Matching dummy commitments let
    /// an exact-balance send be read without input-note history. Other outputs
    /// and public settlements are verified as usual; saved amounts are not
    /// used to infer the result. Unmatched openings cannot hide unreadable
    /// outputs.
    ///
    /// This reads activity details; use [`Self::confirm`] to confirm a payment.
    pub fn transaction_with_prepared(
        &self,
        signature: String,
        pending: &PreparedTransaction,
    ) -> Result<TransactionLookup, WalletError> {
        transaction(self, signature, Some(pending))
    }
}

#[cfg(test)]
mod tests;
