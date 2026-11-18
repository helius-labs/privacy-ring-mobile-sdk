use super::*;
use crate::wallet::prepared::Envelope;
use crate::wallet::tests::{cluster_wallet, config, open, unsigned, GENESIS};
use crate::wallet::WalletConfig;
use solana_keypair::Keypair;
use solana_signer::Signer;
use std::sync::{Arc, Mutex};

fn target() -> CacheTarget {
    CacheTarget {
        sponsor: Pubkey::new_unique().to_string(),
        nonce: 7,
        tree_id: 0,
        expires_at: now() + 600,
        slot: 0,
        max_priority_fee_lamports: 5_000,
        compute_unit_limit: 400_000,
    }
}

#[test]
fn cache_terms_fail_before_network_or_proof() {
    let mut wallet = open(&Keypair::new()).unwrap();
    let mut terms = target();
    terms.slot = CACHE_CAPACITY as u8;
    assert!(wallet
        .prepare_cached_merge(terms, None, None, None, None, None)
        .is_err());
    let mut terms = target();
    terms.expires_at = now() - 1;
    assert!(wallet
        .prepare_cached_merge(terms, None, None, None, None, None)
        .is_err());
    assert!(wallet
        .prepare_cached_merge(target(), None, None, None, Some(5_001), None)
        .is_err());
    let ids = Some(vec![hex::encode([1; 32]), hex::encode([2; 32])]);
    for (mint, max_inputs) in [
        (Some(Pubkey::new_unique().to_string()), None),
        (None, Some(2)),
    ] {
        assert!(wallet
            .prepare_cached_merge(target(), mint, max_inputs, None, None, ids.clone())
            .is_err());
    }
    assert!(wallet
        .prepare_cached_merge(
            target(),
            None,
            None,
            None,
            None,
            Some(vec![hex::encode([1; 32])])
        )
        .is_err());
}

fn on_cluster(signer: &Keypair) -> MobileWallet {
    cluster_wallet(signer, config(), GENESIS).0
}

fn endpoints(rpc_url: &str, indexer_url: &str) -> WalletConfig {
    WalletConfig {
        rpc_url: rpc_url.to_string(),
        indexer_url: indexer_url.to_string(),
        ..config()
    }
}

fn authentication_failed() -> WalletError {
    WalletError::Client {
        message: "snapshot authentication failed".to_string(),
    }
}

#[test]
fn snapshots_authenticate_message_metadata_wallet_and_restore_reservations() {
    let signer = Keypair::new();
    let wallet = on_cluster(&signer);
    let mut pending = unsigned(&signer);
    pending.spends = vec![[9; 32]];
    pending.last_valid_block_height = 123;
    let snapshot = wallet.export_prepared_transaction(&pending).unwrap();
    let reopened = on_cluster(&signer);
    let restored = reopened
        .restore_prepared_transaction(snapshot.clone())
        .unwrap();
    assert_eq!(restored.message(), pending.message());
    assert_eq!(restored.last_valid_block_height(), 123);
    {
        let held = reopened.reservations();
        assert_eq!(held.reserved[&restored.id].spends, vec![[9; 32]]);
        assert_eq!(held.reserved[&restored.id].until, u64::MAX);
    }
    let duplicate = reopened
        .restore_prepared_transaction(snapshot.clone())
        .unwrap();
    assert_eq!(duplicate.id, restored.id);
    assert_eq!(reopened.reservations().reserved.len(), 1);
    let other = on_cluster(&Keypair::new());
    assert_eq!(
        other.restore_prepared_transaction(snapshot.clone()).err(),
        Some(authentication_failed())
    );
    let mut envelope: Envelope = serde_json::from_str(&snapshot).unwrap();
    envelope.payload = envelope.payload.replace("123", "124");
    assert_eq!(
        reopened
            .restore_prepared_transaction(serde_json::to_string(&envelope).unwrap())
            .err(),
        Some(authentication_failed())
    );
    // A different signed message may spend the same inputs. Releasing
    // the first must not release the second after a restart.
    pending.message.set_recent_blockhash([8; 32].into());
    let other_message = reopened
        .restore_prepared_transaction(wallet.export_prepared_transaction(&pending).unwrap())
        .unwrap();
    assert_ne!(other_message.id, restored.id);
    reopened.release(&restored);
    assert_eq!(reopened.reservations().reserved.len(), 1);
    reopened.release(&other_message);
    assert!(reopened.reservations().reserved.is_empty());
}

#[test]
fn snapshots_restore_through_other_endpoints_of_the_deployment() {
    let signer = Keypair::new();
    let (wallet, asked) = cluster_wallet(
        &signer,
        endpoints(
            "https://rpc.example/?api-key=old",
            "https://indexer.example/v1/spr?api-key=old",
        ),
        GENESIS,
    );
    let mut pending = unsigned(&signer);
    pending.spends = vec![[9; 32]];
    let snapshot = wallet.export_prepared_transaction(&pending).unwrap();
    // A rotated API key, another query parameter, and another host of the
    // same cluster, such as a staging edge in front of the same indexer.
    for (rpc_url, indexer_url) in [
        (
            "https://rpc.example/?api-key=new",
            "https://indexer.example/v1/spr?api-key=new",
        ),
        (
            "https://rpc.example/?api-key=old&region=eu",
            "https://indexer.example/v1/spr?api-key=old",
        ),
        (
            "https://rpc-next.example",
            "https://indexer-next.example/v1/spr",
        ),
    ] {
        let (reopened, _) = cluster_wallet(&signer, endpoints(rpc_url, indexer_url), GENESIS);
        let restored = reopened
            .restore_prepared_transaction(snapshot.clone())
            .unwrap();
        assert_eq!(restored.message(), pending.message());
        assert_eq!(
            reopened.reservations().reserved[&restored.id].spends,
            vec![[9; 32]]
        );
    }
    // The wallet reads the genesis hash once.
    wallet.restore_prepared_transaction(snapshot).unwrap();
    assert_eq!(*asked.lock().unwrap(), ["getGenesisHash"]);
}

#[test]
fn snapshots_do_not_restore_on_another_deployment() {
    let signer = Keypair::new();
    let mut pending = unsigned(&signer);
    pending.spends = vec![[9; 32]];
    let snapshot = on_cluster(&signer)
        .export_prepared_transaction(&pending)
        .unwrap();
    let (other_cluster, _) = cluster_wallet(&signer, config(), [6; 32]);
    assert_eq!(
        other_cluster
            .restore_prepared_transaction(snapshot.clone())
            .err(),
        Some(authentication_failed())
    );
    assert!(other_cluster.reservations().reserved.is_empty());
    // Without the genesis hash the wallet cannot tell its deployment: the
    // restore fails as the network does, not as a foreign snapshot.
    let offline = open(&signer).unwrap();
    let error = offline.restore_prepared_transaction(snapshot.clone()).err();
    assert!(
        matches!(&error, Some(WalletError::Client { message }) if message != "snapshot authentication failed"),
        "{error:?}"
    );
    assert!(offline.reservations().reserved.is_empty());
    assert!(offline.export_prepared_transaction(&pending).is_err());
}

#[test]
fn cache_reads_check_owner_terms_commitments_and_duplicate_slots() {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use bytemuck::Zeroable;
    use serde_json::json;
    let signer = Keypair::new();
    let identity = open(&signer).unwrap();
    let terms = target();
    let mint = spr_transaction::Mint::SOL;
    let utxo = Utxo {
        owner: identity.address.signing_pubkey,
        asset: mint,
        amount: 50,
        blinding: [2; 32],
        ring_program_id: None,
        data: Data::default(),
    };
    let commitment = utxo
        .hash(&identity.address.nullifier_pubkey, &[0; 32], &[0; 32], 0)
        .unwrap();
    let output = CachedOutput {
        target: terms.clone(),
        owner: signer.pubkey().to_string(),
        mint: None,
        asset_id: mint.asset_id,
        amount: 50,
        blinding: [2; 32],
        commitment,
    };
    let mut cache = CacheAccount::zeroed();
    cache.discriminator = spr_interface::state::discriminator::CACHE;
    cache.rent_sponsor = parse_pubkey(&terms.sponsor).unwrap();
    cache.write_authority = cache.rent_sponsor;
    cache.expires_at = terms.expires_at.to_le_bytes();
    cache.utxo_hashes[0] = commitment;
    let value = json!({"context":{"slot":1},"value":{"data":[STANDARD.encode(bytemuck::bytes_of(&cache)),"base64"],
        "owner":PROGRAM_ID_PUBKEY.to_string(),"lamports":1,"executable":false,"rentEpoch":0}});
    let response = Arc::new(Mutex::new(value.clone()));
    let shared = response.clone();
    let (wallet, _) = crate::wallet::tests::rpc_wallet(&signer, move |method| {
        if method == "getBlockHeight" {
            return json!({"result":1});
        }
        assert_eq!(method, "getAccountInfo");
        json!({"result":shared.lock().unwrap().clone()})
    });
    wallet.verify_cached_outputs(vec![output.clone()]).unwrap();
    assert!(wallet
        .verify_cached_outputs(vec![output.clone(), output.clone()])
        .is_err());
    let mut changed = output.clone();
    changed.commitment[0] ^= 1;
    assert!(wallet.verify_cached_outputs(vec![changed]).is_err());
    response.lock().unwrap()["value"]["owner"] = json!(Pubkey::new_unique().to_string());
    assert!(wallet.verify_cached_outputs(vec![output.clone()]).is_err());
    *response.lock().unwrap() = value;
    let mut pending = unsigned(&signer);
    pending.cached_output = Some(output.clone());
    let pending = wallet.reserve(pending, vec![[9; 32]]).unwrap();
    let predicted = wallet.cached_note(&output).unwrap().nullifier;
    assert!(wallet.reserved().unwrap().contains(&predicted));
    // Only the real input is a spend reservation: the prediction can be
    // consumed explicitly from cache, but not selected again from the tree.
    assert!(!wallet.reservations().reserved[&pending.id]
        .spends
        .contains(&predicted));
    wallet.release(&pending);
    assert!(!wallet.reserved().unwrap().contains(&predicted));
}

#[test]
fn snapshots_preserve_cache_prediction_and_run_terms() {
    let signer = Keypair::new();
    let wallet = on_cluster(&signer);
    let mut pending = unsigned(&signer);
    let mint = spr_transaction::Mint::SOL;
    let utxo = Utxo {
        owner: wallet.address.signing_pubkey,
        asset: mint,
        amount: 42,
        blinding: [3; 32],
        ring_program_id: None,
        data: Data::default(),
    };
    let commitment = utxo
        .hash(&wallet.address.nullifier_pubkey, &[0; 32], &[0; 32], 0)
        .unwrap();
    pending.cached_output = Some(CachedOutput {
        target: target(),
        owner: signer.pubkey().to_string(),
        mint: None,
        asset_id: mint.asset_id,
        amount: 42,
        blinding: [3; 32],
        commitment,
    });
    let state = wallet.export_prepared_transaction(&pending).unwrap();
    let reopened = on_cluster(&signer);
    let restored = reopened.restore_prepared_transaction(state).unwrap();
    let output = restored.cached_output().unwrap();
    assert_eq!(output.amount(), 42);
    assert_eq!(output.commitment(), commitment.to_vec());
    assert_eq!(output.target(), pending.cached_output().unwrap().target());
    reopened.release(&restored);
    pending.cached_output.as_mut().unwrap().commitment[0] ^= 1;
    assert!(reopened
        .restore_prepared_transaction(wallet.export_prepared_transaction(&pending).unwrap())
        .is_err());
    assert!(reopened.reservations().reserved.is_empty());
}
