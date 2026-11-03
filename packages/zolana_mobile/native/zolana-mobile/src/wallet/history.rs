//! Activity enrichment without fetching every transaction or recipient record.

use std::collections::BTreeSet;

use solana_rpc_client_api::request::RpcRequest;

use super::*;

const BLOCK_TIME_CONCURRENCY: usize = 8;

pub(super) fn entries(
    wallet: &MobileWallet,
    scan: &UtxoScanResult,
) -> Result<Vec<ActivityEntry>, WalletError> {
    let mut entries = activity::entries(&scan.history, &scan.asset_registry)?;
    let missing: Vec<_> = {
        let times = wallet.block_times();
        entries
            .iter()
            .map(|entry| entry.slot)
            .filter(|slot| !times.contains_key(slot))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    };
    for slots in missing.chunks(BLOCK_TIME_CONCURRENCY) {
        let fetched = thread::scope(|scope| {
            let reads: Vec<_> = slots
                .iter()
                .map(|&slot| {
                    scope.spawn(move || {
                        let time = wallet
                            .client
                            .rpc()
                            .client()
                            .send::<Option<i64>>(
                                RpcRequest::GetBlockTime,
                                serde_json::json!([slot]),
                            )
                            .ok()
                            .flatten();
                        (slot, time)
                    })
                })
                .collect();
            reads
                .into_iter()
                .map(|read| read.join().expect("block-time reader panicked"))
                .collect::<Vec<_>>()
        });
        let mut times = wallet.block_times();
        for (slot, time) in fetched {
            if let Some(time) = time {
                times.insert(slot, time);
            }
        }
    }
    let times = wallet.block_times();
    for entry in &mut entries {
        entry.block_time = times.get(&entry.slot).copied();
    }
    Ok(entries)
}
