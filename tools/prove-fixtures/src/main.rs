//! Writes the `/prove` request of each shape named on the command line to
//! `fixtures/`: a confidential transfer for `<inputs>x<outputs>` (`1x2`,
//! `24x2`), a merge for `merge-<inputs>x1` (`merge-24x1`). Each is built as
//! the Zolana client builds one, on trees of this process: the Merkle paths
//! are real and the roots are the trees', so the request proves with the
//! pinned key of its shape, and the proof lands nowhere. The proof timing
//! harness proves these requests on the device.
//!
//! From the repository root:
//! `cargo run --manifest-path tools/prove-fixtures/Cargo.toml -- 1x2 2x2 4x2 8x2 16x2 24x2 merge-24x1`

use std::{env, fs, path::PathBuf, process, sync::Mutex};

use solana_address::Address;
use spr_client::{
    assemble, ClientError, MergeProver, NonInclusionProof, Proof, ProveRequest, Prover, ProverExt,
};
use spr_keypair::ShieldedKeypair;
use spr_transaction::instructions::{
    merge::MergeTransaction,
    transact::{canonical_shape, ConfidentialTransaction},
};

#[path = "../../../fixtures/trees.rs"]
mod fixture_trees;
use fixture_trees::{Trees, TREE_ID};

enum Fixture {
    Transfer { inputs: usize, outputs: usize },
    Merge { inputs: usize },
}

fn main() {
    let fixtures: Vec<(String, Fixture)> = env::args().skip(1).map(parse_fixture).collect();
    if fixtures.is_empty() {
        eprintln!("usage: prove-fixtures <inputs>x<outputs>... merge-<inputs>x1...");
        process::exit(2);
    }
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures");
    for (name, fixture) in fixtures {
        let request = match fixture {
            Fixture::Transfer { inputs, outputs } => transfer_request(inputs, outputs),
            Fixture::Merge { inputs } => merge_request(inputs),
        };
        let path = dir.join(format!("prove-request-{name}.json"));
        fs::write(&path, request.as_bytes()).expect("write the fixture");
        println!("{}: {} bytes", path.display(), request.len());
    }
}

fn parse_fixture(argument: String) -> (String, Fixture) {
    let (shape, merge) = match argument.strip_prefix("merge-") {
        Some(shape) => (shape, true),
        None => (argument.as_str(), false),
    };
    let (inputs, outputs) = shape
        .split_once('x')
        .unwrap_or_else(|| panic!("{argument}: want <inputs>x<outputs> or merge-<inputs>x1"));
    let inputs = inputs.parse().expect("inputs");
    let outputs: usize = outputs.parse().expect("outputs");
    let fixture = if merge {
        assert_eq!(outputs, 1, "{argument}: a merge has one output");
        Fixture::Merge { inputs }
    } else {
        Fixture::Transfer { inputs, outputs }
    };
    (argument, fixture)
}

/// The request of a transfer of half of `inputs` notes of a sender to a
/// recipient, the rest its change, padded to the pinned shape with
/// `outputs` outputs.
fn transfer_request(inputs: usize, outputs: usize) -> String {
    let sender = ShieldedKeypair::new_ed25519().expect("sender");
    let recipient = ShieldedKeypair::new_ed25519().expect("recipient");
    let mut trees = Trees::new();
    let notes = trees.notes(&sender, inputs);

    let payer = Address::new_from_array(
        sender
            .signing_pubkey()
            .as_ed25519()
            .expect("an ed25519 owner"),
    );
    let total: u64 = notes.iter().map(|note| note.utxo.amount).sum();
    let mut transaction = ConfidentialTransaction::new(notes.clone(), payer).expect("transaction");
    transaction
        .transfer_sol(
            &recipient.shielded_address().expect("recipient address"),
            total / 2,
        )
        .expect("transfer");
    transaction
        .pad_utxos(
            canonical_shape(inputs, outputs).expect("a pinned shape"),
            &sender.shielded_address().expect("sender address"),
        )
        .expect("pad to the shape");
    let proof_inputs = transaction.encrypt(&sender).expect("encrypt");

    let spends = trees.spend_proofs(&notes);
    let dummies: Vec<NonInclusionProof> = proof_inputs
        .dummy_nullifiers()
        .into_iter()
        .map(|nullifier| trees.non_inclusion(nullifier))
        .collect();
    let mut assembled = assemble(proof_inputs, &spends, &dummies).expect("assemble");
    let capture = Capture::default();
    // The capture fails the proof on purpose; the request is what we want.
    let _ = assembled.prove(&capture, &sender);
    capture.request()
}

/// The request of a merge of `inputs` notes of one owner into one, as
/// `prepare_merge` builds it.
fn merge_request(inputs: usize) -> String {
    let owner = ShieldedKeypair::new_ed25519().expect("owner");
    let mut trees = Trees::new();
    let notes = trees.notes(&owner, inputs);
    let merge = MergeTransaction::new(notes.clone())
        .expect("merge")
        .with_output_tree_id(TREE_ID)
        .with_expiry(u64::MAX)
        .encrypt(&owner)
        .expect("encrypt");
    let dummy_nullifier_proofs = merge
        .dummy_nullifiers()
        .into_iter()
        .map(|nullifier| trees.non_inclusion(nullifier))
        .collect();
    let result = MergeProver {
        transaction: merge,
        nullifier_key: owner.nullifier_key.clone(),
        proofs: trees.spend_proofs(&notes),
        dummy_nullifier_proofs,
        cache: None,
    }
    .build()
    .expect("build the merge");
    let capture = Capture::default();
    // As for a transfer, the capture fails the proof on purpose.
    let _ = capture.prove_merge(&result.inputs);
    capture.request()
}

/// A prover that keeps the request it is asked for and proves nothing.
#[derive(Default)]
struct Capture {
    body: Mutex<Option<String>>,
}

impl Capture {
    fn request(self) -> String {
        self.body
            .into_inner()
            .expect("lock")
            .expect("the client asked the prover")
    }
}

impl Prover for Capture {
    fn prove(&self, request: &dyn ProveRequest) -> Result<Proof, ClientError> {
        *self.body.lock().expect("lock") = Some(request.body()?.as_str().to_owned());
        Err(ClientError::Rpc("captured".into()))
    }
}
