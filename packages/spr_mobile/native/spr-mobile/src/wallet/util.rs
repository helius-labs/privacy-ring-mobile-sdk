//! Small shared boundary conversions and independent reads.
use crate::error::WalletError;
use solana_pubkey::Pubkey;
use spr_transaction::WalletUtxo;
use std::{collections::HashSet, str::FromStr, thread};

/// Both, `first` on a thread of its own while `second` runs here: the
/// network calls of a spend that need nothing from each other.
pub(super) fn beside<A: Send, B>(
    first: impl FnOnce() -> A + Send,
    second: impl FnOnce() -> B,
) -> (A, B) {
    thread::scope(|scope| {
        let first = scope.spawn(first);
        let second = second();
        let first = first
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
        (first, second)
    })
}

/// Whether a spend may select `note`: a positive note the default-ring
/// circuit spends that no prepared spend holds. The Zolana SDK's selection
/// takes the same candidates; its filter is not public.
pub(super) fn spend_candidate(note: &WalletUtxo, reserved: &HashSet<[u8; 32]>) -> bool {
    note.utxo.amount > 0 && note.is_default_ring_spendable() && !reserved.contains(&note.nullifier)
}

pub(super) fn nullifiers(notes: &[WalletUtxo]) -> Vec<[u8; 32]> {
    notes.iter().map(|note| note.nullifier).collect()
}

pub(super) fn parse_pubkey(value: &str) -> Result<Pubkey, WalletError> {
    Pubkey::from_str(value).map_err(|_| WalletError::InvalidPubkey {
        value: value.to_string(),
    })
}

pub(super) fn invalid(message: &str) -> WalletError {
    WalletError::Client {
        message: message.to_owned(),
    }
}
