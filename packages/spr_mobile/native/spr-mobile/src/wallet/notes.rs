//! Public note descriptions and exact merge input selection. Private openings
//! stay in the wallet; the application refers to notes by their commitments.
use super::{util::spend_candidate, MobileWallet};
use crate::{asset::mint_name, error::WalletError};
use spr_transaction::WalletUtxo;
use std::collections::{HashMap, HashSet};

/// An unreserved, nonzero default-ring note from a confirmed indexer scan.
/// This description does not reserve the note or guarantee future availability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpendableNote {
    /// The published commitment as 64 lowercase hex characters. Stable across
    /// scans and restarts of the same wallet on the same deployment.
    pub id: String,
    /// `None` for SOL; otherwise the SPL mint's base58 address.
    pub mint: Option<String>,
    /// Integer base units, never a floating-point token amount.
    pub amount: u64,
    pub tree_id: u16,
    /// Merges require plain notes, without attached data or ring bindings.
    pub mergeable: bool,
}

impl MobileWallet {
    /// Read spendable notes now, excluding this wallet's reservations,
    /// including predicted cache outputs. As with balances, unresolved assets
    /// are omitted. Restore pending transactions before listing notes after a
    /// restart.
    pub fn spendable_notes(&mut self) -> Result<Vec<SpendableNote>, WalletError> {
        let spendable = self.spendable()?;
        Ok(describe_notes(spendable.utxos(), &self.reserved()?))
    }
}

pub(super) fn describe_notes<'a>(
    notes: impl Iterator<Item = &'a WalletUtxo>,
    reserved: &HashSet<[u8; 32]>,
) -> Vec<SpendableNote> {
    notes
        .filter(|note| spend_candidate(note, reserved))
        .map(|note| SpendableNote {
            id: hex::encode(note.utxo_hash),
            mint: mint_name(&note.utxo.asset.asset),
            amount: note.utxo.amount,
            tree_id: note.tree_id(),
            mergeable: note.is_plain(),
        })
        .collect()
}

/// CreateCache + MergeTransact fits at most 51 real inputs in a 4096-byte v1
/// message. The compiled transaction is also checked before it is returned.
pub(super) const MAX_CACHED_MERGE_INPUTS: usize = 51;

/// Parse ordered note IDs: 2 to [`MAX_CACHED_MERGE_INPUTS`] distinct
/// lowercase hex commitments.
pub(super) fn merge_input_ids(ids: &[String]) -> Result<Vec<[u8; 32]>, WalletError> {
    if !(2..=MAX_CACHED_MERGE_INPUTS).contains(&ids.len()) {
        return Err(WalletError::MergeInputCountInvalid {
            count: ids.len() as u64,
            max: MAX_CACHED_MERGE_INPUTS as u64,
        });
    }
    let mut seen = HashSet::new();
    ids.iter()
        .map(|id| {
            let mut hash = [0; 32];
            if id.bytes().any(|byte| byte.is_ascii_uppercase())
                || hex::decode_to_slice(id, &mut hash).is_err()
                || !seen.insert(hash)
            {
                return Err(WalletError::NoteIdInvalid { id: id.clone() });
            }
            Ok(hash)
        })
        .collect()
}

/// The notes `ids` name, in that order. A missing, reserved, off-tree or
/// non-plain note fails; nothing is substituted. Mixed mints fail when the
/// merge is built.
pub(super) fn select_merge_inputs<'a>(
    notes: impl Iterator<Item = &'a WalletUtxo>,
    ids: &[[u8; 32]],
    tree_id: u16,
    reserved: &HashSet<[u8; 32]>,
) -> Result<Vec<WalletUtxo>, WalletError> {
    let by_hash: HashMap<_, _> = notes.map(|note| (note.utxo_hash, note)).collect();
    ids.iter()
        .map(|id| {
            let note = by_hash
                .get(id)
                .ok_or_else(|| WalletError::MergeInputUnavailable {
                    id: hex::encode(id),
                })?;
            if reserved.contains(&note.nullifier) {
                return Err(WalletError::NotesReserved {
                    amount: note.utxo.amount,
                });
            }
            if note.tree_id() != tree_id || note.utxo.amount == 0 || !note.is_plain() {
                return Err(WalletError::MergeInputInvalid {
                    id: hex::encode(id),
                });
            }
            Ok((*note).clone())
        })
        .collect()
}

#[cfg(test)]
mod tests;
