//! On-device proving with a shared, memory-bounded loaded-key slot.

use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Mutex, PoisonError,
    },
    time::Instant,
};

use zeroize::Zeroizing;

use image::load_key;

mod image;

pub use image::image_path;

/// Use this feature in the final binary to release large freed blocks to the
/// OS on Apple targets. It installs the allocator for the whole process.
#[cfg(feature = "device-allocator")]
#[global_allocator]
static ALLOCATOR: zopro_spr::zopro_core::alloc::Allocator = zopro_spr::zopro_core::alloc::Allocator;

#[derive(Debug, thiserror::Error)]
pub enum ProverError {
    #[error("the prover is busy")]
    Busy,
    #[error("the loaded prover is closed")]
    Closed,
    #[error("the prover state is unavailable")]
    Unavailable,
    #[error("the proving key could not be loaded")]
    LoadFailed,
    #[error("the circuit is unsupported")]
    UnsupportedCircuit,
    #[error("the proof failed")]
    ProofFailed,
}

static PREPARED: Mutex<ProverState> = Mutex::new(ProverState {
    next_id: 1,
    loaded: None,
});

/// The one proving key loaded at a time: a phone has room for one.
struct ProverState {
    next_id: u64,
    loaded: Option<Loaded>,
}

struct Loaded {
    id: u64,
    /// A reusable file path, or `None` for an exclusive [`LocalProver`].
    path: Option<PathBuf>,
    prover: zopro_spr::Prover,
}

impl ProverState {
    fn next_id(&mut self) -> Result<u64, ProverError> {
        let id = self.next_id;
        self.next_id = id.checked_add(1).ok_or(ProverError::Unavailable)?;
        Ok(id)
    }
}

/// A verified proof and where its time went, in milliseconds.
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

impl LocalProofResult {
    fn from_proof(
        proof: zopro_spr::ProofResult,
        inputs: u32,
        outputs: u32,
        started: Instant,
    ) -> Self {
        Self {
            proof_json: proof.proof_json.to_string(),
            verified: true,
            inputs,
            outputs,
            proof_ms: proof.prove_ms,
            witness_ms: proof.witness_ms,
            verify_ms: proof.verify_ms,
            total_ms: elapsed_ms(started),
        }
    }
}

/// A zopro key the application loaded from a file, to prove structured
/// Zolana `/prove` requests with on the device, such as for a benchmark.
/// One key is loaded at a time: while this one is, other consumers' proofs
/// fail with [`ProverError::Busy`].
pub struct LocalProver {
    id: u64,
    load_ms: u64,
    closed: AtomicBool,
}

impl LocalProver {
    /// Load a trusted zopro key at `key_path`. Keep its contents immutable.
    /// Fails with [`ProverError::Busy`] while another key is loaded or proving.
    pub fn load(key_path: &Path) -> Result<LocalProver, ProverError> {
        let started = Instant::now();
        let mut state = PREPARED.try_lock().map_err(|_| ProverError::Busy)?;
        if state.loaded.is_some() {
            return Err(ProverError::Busy);
        }
        let id = state.next_id()?;
        let prover = load_key(key_path)?;
        state.loaded = Some(Loaded {
            id,
            path: None,
            prover,
        });
        Ok(LocalProver {
            id,
            load_ms: elapsed_ms(started),
            closed: AtomicBool::new(false),
        })
    }

    /// How long loading the key took.
    pub fn load_ms(&self) -> u64 {
        self.load_ms
    }

    /// Prove `request_json`, a Zolana `/prove` request; the proof is
    /// verified. One proof at a time: another fails with `prover_busy`.
    /// Fails with `prover_closed` after `close`, also when the close came
    /// while this proof ran.
    pub fn prove(&self, request_json: String) -> Result<LocalProofResult, ProverError> {
        let request_json = Zeroizing::new(request_json);
        let started = Instant::now();
        if self.closed.load(Ordering::Acquire) {
            return Err(ProverError::Closed);
        }
        let mut state = PREPARED.try_lock().map_err(|_| ProverError::Busy)?;
        let loaded = state
            .loaded
            .as_mut()
            .filter(|loaded| loaded.id == self.id)
            .ok_or(ProverError::Closed)?;
        let (inputs, outputs) = loaded.prover.shape();
        let proof = loaded.prover.prove(&request_json);
        if self.closed.load(Ordering::Acquire) {
            state.loaded = None;
            return Err(ProverError::Closed);
        }
        let proof = proof.map_err(|error| match error {
            zopro_spr::Error::Unsupported | zopro_spr::Error::Shape => {
                ProverError::UnsupportedCircuit
            }
            _ => ProverError::ProofFailed,
        })?;
        Ok(LocalProofResult::from_proof(
            proof, inputs, outputs, started,
        ))
    }

    /// Release the key, after the proof in progress, which then fails with
    /// `prover_closed`. Later proofs fail the same way.
    pub fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.release();
    }

    fn release(&self) {
        let mut state = PREPARED.lock().unwrap_or_else(PoisonError::into_inner);
        if state
            .loaded
            .as_ref()
            .is_some_and(|loaded| loaded.id == self.id)
        {
            state.loaded = None;
        }
    }
}

impl Drop for LocalProver {
    fn drop(&mut self) {
        self.release();
    }
}

/// Reuses the process-wide slot for a series of proofs, switching keys as needed.
/// Proving fails with [`ProverError::Busy`] while a [`LocalProver`] holds the slot.
#[derive(Default)]
pub struct ProverSession {
    loaded: AtomicU64,
}

impl ProverSession {
    /// Prove with a trusted, immutable key file. Its path identifies the
    /// reusable key. Returns the verified proof and time spent loading the key.
    pub fn prove(
        &self,
        request_json: &str,
        key_path: &Path,
    ) -> Result<(LocalProofResult, u64), ProverError> {
        let started = Instant::now();
        let mut state = PREPARED.lock().map_err(|_| ProverError::Unavailable)?;
        let loading = Instant::now();
        match state.loaded.as_ref().map(|loaded| loaded.path.as_deref()) {
            Some(None) => return Err(ProverError::Busy),
            Some(Some(path)) if path == key_path => {}
            _ => {
                // Free the old key before allocating memory for the new one.
                state.loaded = None;
                let prover = load_key(key_path)?;
                let id = state.next_id()?;
                self.loaded.store(id, Ordering::Relaxed);
                state.loaded = Some(Loaded {
                    id,
                    path: Some(key_path.to_path_buf()),
                    prover,
                });
            }
        }
        let load_ms = elapsed_ms(loading);
        let loaded = state.loaded.as_mut().ok_or(ProverError::Closed)?;
        let (inputs, outputs) = loaded.prover.shape();
        let proof = loaded
            .prover
            .prove(request_json)
            .map_err(|error| match error {
                zopro_spr::Error::Unsupported => ProverError::UnsupportedCircuit,
                zopro_spr::Error::Prove(message) => {
                    log::warn!("zopro: {message}");
                    ProverError::ProofFailed
                }
                _ => ProverError::ProofFailed,
            })?;
        Ok((
            LocalProofResult::from_proof(proof, inputs, outputs, started),
            load_ms,
        ))
    }
}

impl Drop for ProverSession {
    fn drop(&mut self) {
        let id = *self.loaded.get_mut();
        let Ok(mut state) = PREPARED.lock() else {
            return;
        };
        if state.loaded.as_ref().is_some_and(|loaded| loaded.id == id) {
            state.loaded = None;
        }
    }
}

fn elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().try_into().unwrap_or(u64::MAX)
}
