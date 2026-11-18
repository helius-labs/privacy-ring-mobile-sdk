use std::path::Path;

use spr_hasher::{Hasher, Poseidon};

mod activity;
mod asset;
mod error;
mod prover;
mod proving_keys;
mod transport;
mod wallet;

pub use activity::{
    ActivityEntry, ActivityKind, TransactionLookup, TransactionLookupStatus, TransactionOutput,
};
pub use asset::MintConfig;
pub use error::WalletError;
pub use prover::Proving;
pub use proving_keys::{
    ProvingKeyProgress, ProvingKeyStatus, ProvingKeys, DEFAULT_PROVING_KEYS_URL,
};
pub use transport::{
    ProverTeeNetwork, Transport, TransportFailure, TransportOutcome, TransportRequest,
    TransportResponse,
};
pub use wallet::{
    derivation_message, CacheTarget, CachedOutput, MobileWallet, Operation, Output, OutputOpening,
    PreparedTransaction, RegistrationStatus, SpendLimit, SpendableNote, TokenBalance,
    UnresolvedAssets, WalletConfig, WalletKeys, WalletOverview,
};

/// A proof of a `/prove` request through the wallet's key store, by
/// [`ProvingKeys::prove`], and where its time went. Milliseconds; the
/// stages add up to about `total_ms`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvedRequest {
    pub proof_json: String,
    /// The key it was proved with.
    pub key: String,
    /// Bytes of the key downloaded for this proof: its size when it was not
    /// on the device, what a partial download lacked, or none.
    pub downloaded_bytes: u64,
    /// Fetching and checking the key, download included.
    pub download_ms: u64,
    /// Loading the key into the prover; none when it was the loaded one.
    pub load_ms: u64,
    pub witness_ms: u64,
    pub prove_ms: u64,
    pub verify_ms: u64,
    pub total_ms: u64,
}

pub fn sdk_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn poseidon_hash(inputs: Vec<Vec<u8>>) -> Result<Vec<u8>, WalletError> {
    if inputs.is_empty() || inputs.len() > 12 {
        return Err(WalletError::PoseidonInputCountInvalid {
            count: inputs.len() as u64,
        });
    }
    if let Some((index, input)) = inputs
        .iter()
        .enumerate()
        .find(|(_, input)| input.len() != 32)
    {
        return Err(WalletError::PoseidonInputLengthInvalid {
            index: index as u64,
            length: input.len() as u64,
        });
    }
    let refs = inputs.iter().map(Vec::as_slice).collect::<Vec<_>>();
    Poseidon::hashv(&refs)
        .map(|hash| hash.to_vec())
        .map_err(|failure| spr_client::ClientError::from(failure).into())
}

/// A proof by a [`LocalProver`] and where its time went, in milliseconds.
#[derive(Debug, Clone)]
pub struct LocalProofResult {
    pub proof_json: String,
    /// Every proof is verified before it is returned.
    pub verified: bool,
    pub inputs: u32,
    pub outputs: u32,
    pub proof_ms: u64,
    pub witness_ms: u64,
    pub verify_ms: u64,
    pub total_ms: u64,
}

/// A zopro key the application loaded from a file, to prove structured
/// Zolana `/prove` requests with on the device, such as for a benchmark.
/// One key is loaded at a time: while this one is, the wallet's own proofs
/// fail with `prover_busy`, so close it before the wallet proves.
pub struct LocalProver {
    inner: spr_local_prover::LocalProver,
}

impl LocalProver {
    /// Load the zopro key at `key_path`. Fails with `prover_busy` while
    /// another key is loaded or proving.
    pub fn load(key_path: String) -> Result<LocalProver, WalletError> {
        Ok(Self {
            inner: spr_local_prover::LocalProver::load(Path::new(&key_path))?,
        })
    }

    /// How long loading the key took.
    #[flutter_rust_bridge::frb(sync, getter)]
    pub fn load_ms(&self) -> u64 {
        self.inner.load_ms()
    }

    /// Prove `request_json`, a Zolana `/prove` request; the proof is
    /// verified. One proof at a time: another fails with `prover_busy`.
    /// Fails with `prover_closed` after `close`, also when the close came
    /// while this proof ran.
    pub fn prove(&self, request_json: String) -> Result<LocalProofResult, WalletError> {
        let proof = self.inner.prove(request_json)?;
        Ok(LocalProofResult {
            proof_json: proof.proof_json,
            verified: proof.verified,
            inputs: proof.inputs,
            outputs: proof.outputs,
            proof_ms: proof.proof_ms,
            witness_ms: proof.witness_ms,
            verify_ms: proof.verify_ms,
            total_ms: proof.total_ms,
        })
    }

    /// Release the key, after the proof in progress, which then fails with
    /// `prover_closed`. Later proofs fail the same way.
    pub fn close(&self) {
        self.inner.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poseidon_binding_validates_inputs() {
        assert!(poseidon_hash(vec![]).is_err());
        assert!(poseidon_hash(vec![vec![0; 31]]).is_err());
        assert_eq!(poseidon_hash(vec![vec![0; 32]]).unwrap().len(), 32);
    }
}
