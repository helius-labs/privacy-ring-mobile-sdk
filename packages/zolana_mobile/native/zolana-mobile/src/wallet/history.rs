//! Activity enrichment without fetching every transaction or recipient record.

use std::{
    collections::BTreeSet,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

use solana_rpc_client_api::request::RpcRequest;
use zolana_client::UtxoScanResult;

use super::MobileWallet;
use crate::{
    activity::{self, ActivityEntry},
    error::WalletError,
};

/// Concurrent `getBlockTime` reads.
const BLOCK_TIME_CONCURRENCY: usize = 8;
/// Slots read per activity call, newest first. Older rows stay without a
/// time until a later read, so a long history does not hold up balances.
const BLOCK_TIME_READS: usize = 32;
/// The wait before a slot without a time is asked for again. It doubles on
/// every further miss, up to [`BLOCK_TIME_RETRY_MAX`]: a non-archival RPC
/// has no time for slots past its retention, on every read.
const BLOCK_TIME_RETRY: Duration = Duration::from_secs(30);
const BLOCK_TIME_RETRY_MAX: Duration = Duration::from_secs(60 * 60);

/// What the RPC said about a slot's time, shared by activity and lookup.
pub(super) enum BlockTime {
    Known(i64),
    /// No time, or the read failed; not asked again before `retry_at`.
    Missing {
        retry_at: Instant,
        wait: Duration,
    },
}

pub(super) fn entries(
    wallet: &MobileWallet,
    scan: &UtxoScanResult,
) -> Result<Vec<ActivityEntry>, WalletError> {
    let mut entries = activity::entries(&scan.history, &scan.asset_registry)?;
    let now = Instant::now();
    let missing: Vec<_> = {
        let times = wallet.block_times();
        entries
            .iter()
            .map(|entry| entry.slot)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .rev()
            .filter(|slot| match times.get(slot) {
                None => true,
                Some(BlockTime::Known(_)) => false,
                Some(BlockTime::Missing { retry_at, .. }) => *retry_at <= now,
            })
            .take(BLOCK_TIME_READS)
            .collect()
    };
    if !missing.is_empty() {
        let read = |slot: u64| {
            wallet
                .client
                .rpc()
                .client()
                .send::<Option<i64>>(RpcRequest::GetBlockTime, serde_json::json!([slot]))
                .ok()
                .flatten()
        };
        // A fixed pool of readers taking the next slot each, rather than
        // chunks that respawn and wait for their slowest read.
        let next = AtomicUsize::new(0);
        let fetched = thread::scope(|scope| {
            let readers: Vec<_> = (0..BLOCK_TIME_CONCURRENCY.min(missing.len()))
                .map(|_| {
                    scope.spawn(|| {
                        let mut times = Vec::new();
                        while let Some(&slot) = missing.get(next.fetch_add(1, Ordering::Relaxed)) {
                            times.push((slot, read(slot)));
                        }
                        times
                    })
                })
                .collect();
            readers
                .into_iter()
                .flat_map(|reader| reader.join().expect("block-time reader panicked"))
                .collect::<Vec<_>>()
        });
        let mut times = wallet.block_times();
        for (slot, time) in fetched {
            let time = match time {
                Some(time) => BlockTime::Known(time),
                None => {
                    let wait = match times.get(&slot) {
                        Some(BlockTime::Missing { wait, .. }) => {
                            (*wait * 2).min(BLOCK_TIME_RETRY_MAX)
                        }
                        _ => BLOCK_TIME_RETRY,
                    };
                    BlockTime::Missing {
                        retry_at: Instant::now() + wait,
                        wait,
                    }
                }
            };
            times.insert(slot, time);
        }
    }
    let times = wallet.block_times();
    for entry in &mut entries {
        entry.block_time = match times.get(&entry.slot) {
            Some(BlockTime::Known(time)) => Some(*time),
            _ => None,
        };
    }
    Ok(entries)
}
