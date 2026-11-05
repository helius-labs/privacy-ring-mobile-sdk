//! The wallet's history as the Zolana SDK reads and classifies it, in the
//! types the bindings carry.

use std::collections::{BTreeSet, HashMap, HashSet};

use solana_pubkey::Pubkey;
use spr_client::rpc::OutputSlot;
use spr_interface::event::OutputDataEncoding;
use spr_transaction::{
    AssetRegistry, EncryptedScheme, History, HistoryEntry, HistoryKind, ShieldedTransaction,
};

use crate::{asset::mint_name, error::WalletError};

/// What a history entry did, from this wallet's side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivityKind {
    /// Public funds moved into the private balance.
    Deposit,
    /// A transfer from another wallet.
    Received,
    /// A transfer to another wallet.
    Sent,
    /// Private funds moved to a public account.
    Withdrawal,
    /// Notes moved within this wallet: a merge, or a transfer to itself.
    SelfTransfer,
}

/// Amounts are in base units: lamports for SOL, the mint's smallest unit
/// otherwise. `mint` is `None` for SOL. Sent and withdrawn amounts are what
/// left the private balance, change excluded; a self transfer's is what it
/// spent.
///
/// The optional block time is in Unix seconds. Outgoing recipients are the
/// sorted, unique private addresses the signature's non-own output tags name,
/// the same from every read. See `TransactionLookup.outputs` for public
/// destinations and individual amounts.
/// The stable `index` is the pool's asset id (1 for SOL); together with
/// `signature` and `slot`, it identifies the same asset row across reads.
#[derive(Clone, Debug, PartialEq, Eq)]
#[flutter_rust_bridge::frb(non_opaque, dart_metadata = ("freezed"))]
pub struct ActivityEntry {
    pub kind: ActivityKind,
    pub mint: Option<String>,
    pub amount: u64,
    pub signature: String,
    pub slot: u64,
    /// Estimated block production time, in Unix seconds. `None` when the RPC
    /// has no timestamp for this slot or the time lookup failed.
    pub block_time: Option<i64>,
    /// Private recipient addresses from this signature's non-own output
    /// tags, sorted and deduplicated. Empty for incoming and self transfers.
    /// Tags do not identify assets, so a multi-asset signature repeats the
    /// same addresses on each outgoing row, in activity, overview and
    /// transaction lookup alike. Public recipients and per-output amounts
    /// are in `TransactionLookup.outputs`.
    #[frb(default = "const []")]
    pub recipients: Vec<String>,
    /// Stable asset-row index: the pool's asset id (1 for SOL), not the row's
    /// position in a list or the TS SDK's output-leaf index. Together with
    /// `signature` and `slot`, this identifies the same row across reads.
    pub index: u64,
}

/// `history` newest first: one entry per asset a transaction moved.
pub(crate) fn entries(
    history: &History,
    assets: &AssetRegistry,
) -> Result<Vec<ActivityEntry>, WalletError> {
    let owned: HashSet<_> = history.utxos.iter().map(|note| note.utxo_hash).collect();
    let mut recipients = HashMap::<_, BTreeSet<_>>::new();
    for tx in &history.transactions {
        recipients
            .entry(tx.tx_signature)
            .or_default()
            .extend(self::recipients(tx, |slot| {
                history.view_tags.contains(&slot.view_tag)
                    || owned.contains(&slot.output_context.hash)
            }));
    }
    history
        .entries()
        .into_iter()
        .map(|entry| {
            let addresses = recipients.get(&entry.tx_signature);
            let mut entry = activity_entry(entry, assets)?;
            if matches!(entry.kind, ActivityKind::Sent | ActivityKind::Withdrawal) {
                entry.recipients = addresses.into_iter().flatten().cloned().collect();
            }
            Ok(entry)
        })
        .collect()
}

/// The private addresses `tx`'s output tags name, `own` outputs excluded:
/// the one rule activity, overview and transaction lookup share. Ring
/// discovery tags are not recipient Solana addresses.
pub(crate) fn recipients<'a>(
    tx: &'a ShieldedTransaction,
    own: impl Fn(&OutputSlot) -> bool + 'a,
) -> impl Iterator<Item = String> + 'a {
    let ring = tx.ring_config.is_some() || tx.ring_program_id.is_some();
    tx.output_slots
        .iter()
        .filter(move |slot| {
            !ring
                && !own(slot)
                && matches!(slot.output_data(), Some(OutputDataEncoding::Encrypted(blob))
                    if blob.first() == Some(&EncryptedScheme::Confidential.as_byte()))
        })
        .map(|slot| Pubkey::new_from_array(slot.view_tag).to_string())
}

pub(crate) fn activity_entry(
    entry: HistoryEntry,
    assets: &AssetRegistry,
) -> Result<ActivityEntry, WalletError> {
    let index = assets.asset_id(&entry.mint)?;
    Ok(ActivityEntry {
        kind: match entry.kind {
            HistoryKind::Deposit => ActivityKind::Deposit,
            HistoryKind::Received => ActivityKind::Received,
            HistoryKind::Sent => ActivityKind::Sent,
            HistoryKind::Withdrawal => ActivityKind::Withdrawal,
            HistoryKind::SelfTransfer => ActivityKind::SelfTransfer,
        },
        mint: mint_name(&entry.mint),
        amount: entry.amount,
        signature: entry.tx_signature.to_string(),
        slot: entry.slot,
        block_time: None,
        recipients: vec![],
        index,
    })
}

/// Whether a signature has a complete wallet result at the indexed slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransactionLookupStatus {
    /// The complete result for configured assets. `entries` may be empty.
    Ready,
    /// The confirmed transaction's slot is ahead of the indexer. Retry later.
    PendingIndexing,
    /// Neither the indexer nor the confirmed RPC returned the transaction.
    /// This does not prove it never existed: the RPC may prune old history.
    NotFound,
    /// The transaction was found, but this SDK cannot reconstruct every
    /// wallet row from its format, ciphertexts or available RPC metadata.
    /// No partial entries are returned and no history scan is attempted.
    Incomplete,
}

/// One outgoing payment, with change and padding excluded. Private outputs
/// and public withdrawals remain separate, even when they pay the same
/// address. An application's privacy fee is an ordinary private output to
/// its collector; the SDK does not infer which payment is a fee.
#[derive(Clone, Debug, PartialEq, Eq)]
#[flutter_rust_bridge::frb(non_opaque)]
pub struct TransactionOutput {
    /// Destination Solana address. For a public SPL withdrawal this is the
    /// settlement token account, not its owner.
    pub recipient: String,
    /// `None` for SOL. Amounts are in base units.
    pub mint: Option<String>,
    pub amount: u64,
    /// `true` for a public withdrawal, `false` for a private note.
    pub is_public: bool,
}

/// A signature-scoped read. Network failures throw; they are never empty
/// wallet results. Only `ready` may carry entries and outputs. Chain metadata
/// may be available in other statuses too.
#[derive(Clone, Debug, PartialEq, Eq)]
#[flutter_rust_bridge::frb(non_opaque, dart_metadata = ("freezed"))]
pub struct TransactionLookup {
    pub status: TransactionLookupStatus,
    pub entries: Vec<ActivityEntry>,
    /// Each outgoing private payment and public withdrawal, without merging
    /// payments by recipient or asset. Empty for incoming and self transfers.
    /// Private payments precede public withdrawals within each event.
    #[frb(default = "const []")]
    pub outputs: Vec<TransactionOutput>,
    pub slot: Option<u64>,
    /// Estimated block production time, in Unix seconds, from `getTransaction`.
    pub block_time: Option<i64>,
    /// Whole Solana transaction network fee in lamports, from `meta.fee`,
    /// including any priority fee. Not a privacy fee or an asset-row amount.
    pub fee: Option<u64>,
    /// Highest slot the indexer had persisted for this lookup.
    pub indexed_slot: u64,
}
