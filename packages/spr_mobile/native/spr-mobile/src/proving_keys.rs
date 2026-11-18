//! Proving-key helpers exposed by the Flutter wallet.

use std::{
    collections::{BTreeSet, HashMap},
    io,
    path::PathBuf,
    sync::Arc,
};

use flutter_rust_bridge::DartFnFuture;
use spr_local_prover::{pinned_key, pinned_key_size, KeyDownloadResponse};
use spr_transaction::instructions::{merge::merge_circuit_width, transact::canonical_shape};
use zeroize::Zeroizing;

use crate::{
    error::WalletError,
    prover::NativeProver,
    transport::{Transport, TransportRequest},
    ProvedRequest,
};

pub(crate) use spr_local_prover::KeyStore;
pub use spr_local_prover::DEFAULT_PROVING_KEYS_URL;

pub(crate) fn key_store(
    root: impl Into<PathBuf>,
    base_url: Option<String>,
    transport: Transport,
) -> KeyStore {
    KeyStore::new(
        root,
        base_url,
        Arc::new(move |request| {
            let response = transport
                .send_blocking(TransportRequest {
                    method: "GET".to_string(),
                    url: request.url,
                    headers: HashMap::from([(
                        "range".to_string(),
                        format!("bytes={}-{}", request.range.start(), request.range.end()),
                    )]),
                    body: Vec::new(),
                    max_response_bytes: request.max_response_bytes,
                    timeout_ms: None,
                    download_path: Some(request.destination.display().to_string()),
                })
                .map_err(|_| io::Error::other("key transport failed"))?;
            Ok(KeyDownloadResponse {
                status: response.status,
                headers: response.headers,
                body: response.body,
            })
        }),
    )
}

/// A proving key of the wallet and how much of it is on the device.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvingKeyStatus {
    pub name: String,
    /// The key's size in the lockfile.
    pub size: u64,
    /// Bytes on the device: `size` once it downloaded, what an interrupted
    /// download left otherwise.
    pub downloaded: u64,
}

/// Where a [`ProvingKeys::prefetch`] is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProvingKeyProgress {
    /// The key downloading now.
    pub name: String,
    pub key_downloaded: u64,
    pub key_size: u64,
    /// Keys complete so far, of `keys_total` that were not on the device
    /// when the prefetch started.
    pub keys_done: u32,
    pub keys_total: u32,
    /// Bytes on the device of those keys, of `total`.
    pub downloaded: u64,
    pub total: u64,
}

/// The wallet's proving keys: which are on the device, a download ahead of
/// the first proof that needs them, and their removal. It works beside the
/// wallet's other calls: a proof that needs a key a prefetch is downloading
/// waits for that download instead of starting its own.
pub struct ProvingKeys {
    pub(crate) store: Arc<KeyStore>,
    /// The wallet's own prover, so that a key it loaded stays loaded.
    pub(crate) prover: NativeProver,
}

impl ProvingKeys {
    /// Every key the wallet can prove with on the device, smallest first.
    pub fn status(&self) -> Vec<ProvingKeyStatus> {
        self.store
            .status()
            .into_iter()
            .map(|key| ProvingKeyStatus {
                name: key.name,
                size: key.size,
                downloaded: key.downloaded,
            })
            .collect()
    }

    /// The keys spends of up to `max_inputs` notes with `outputs` outputs,
    /// and merges of up to `max_merge_inputs` notes, prove with, smallest
    /// first. A transfer has two outputs, the payment and the change; a
    /// withdrawal pads its change to two.
    pub fn needed(
        &self,
        max_inputs: u32,
        outputs: Vec<u32>,
        max_merge_inputs: u32,
    ) -> Result<Vec<String>, WalletError> {
        let mut names = BTreeSet::new();
        for n_in in 1..=max_inputs as usize {
            for &n_out in &outputs {
                let shape = canonical_shape(n_in, n_out as usize).map_err(|_| {
                    WalletError::ProvingKeyUnknown {
                        name: format!("transfer_confidential_{n_in}_{n_out}.key"),
                    }
                })?;
                names.insert(format!(
                    "transfer_confidential_{}_{}.key",
                    shape.n_inputs(),
                    shape.n_outputs()
                ));
            }
        }
        for count in 2..=max_merge_inputs as usize {
            let width = merge_circuit_width(count).ok_or(WalletError::ProvingKeyUnknown {
                name: format!("merge_{count}_1.key"),
            })?;
            names.insert(format!("merge_{width}_1.key"));
        }
        let mut sized = names
            .into_iter()
            .map(|name| Ok((pinned_key_size(&name)?, name)))
            .collect::<Result<Vec<_>, WalletError>>()?;
        sized.sort_by_key(|(size, _)| *size);
        Ok(sized.into_iter().map(|(_, name)| name).collect())
    }

    /// Download `names` (see [`Self::needed`]) that are not on the device,
    /// each checked against the lockfile. `progress` hears of every part, and
    /// stops the prefetch by answering `false`; what it downloaded stays, and
    /// the next download of that key resumes from it. Returns whether every
    /// key is on the device.
    pub fn prefetch(
        &self,
        names: Vec<String>,
        progress: impl Fn(ProvingKeyProgress) -> DartFnFuture<bool> + Send + Sync + 'static,
    ) -> Result<bool, WalletError> {
        Ok(self.store.prefetch(&names, |update| {
            futures_executor::block_on(progress(ProvingKeyProgress {
                name: update.name.clone(),
                key_downloaded: update.key_downloaded,
                key_size: update.key_size,
                keys_done: update.keys_done,
                keys_total: update.keys_total,
                downloaded: update.downloaded,
                total: update.total,
            }))
        })?)
    }

    /// Remove the keys and partial downloads from the device. A prefetch or
    /// proof that is downloading a key finishes its part first.
    pub fn clear(&self) -> Result<(), WalletError> {
        self.store.clear().map_err(Into::into)
    }

    /// Prove `request_json`, a Zolana `/prove` request, with the pinned key
    /// `key_name` (such as `transfer_confidential_1_2.key`), as the wallet
    /// proves a spend: the key downloads when it is not on the device, loads
    /// when it is not the loaded one, and the proof is verified. The result
    /// says where the time went, so an application can time proofs on a
    /// device with a fixed request. One proof at a time with the wallet's
    /// own; a `LocalProver` loaded meanwhile fails it with `prover_busy`.
    pub fn prove(
        &self,
        key_name: String,
        request_json: String,
    ) -> Result<ProvedRequest, WalletError> {
        let request_json = Zeroizing::new(request_json);
        self.prover
            .prove_request(&request_json, &pinned_key(&key_name)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::tests::unreachable;

    fn needed(
        max_inputs: u32,
        outputs: Vec<u32>,
        max_merge_inputs: u32,
    ) -> Result<Vec<String>, WalletError> {
        let root = std::env::temp_dir().join(format!("spr-keys-needed-{}", std::process::id()));
        let store = Arc::new(key_store(root, None, unreachable()));
        ProvingKeys {
            prover: NativeProver::new(Arc::clone(&store)),
            store,
        }
        .needed(max_inputs, outputs, max_merge_inputs)
    }

    #[test]
    fn every_transfer_shape_has_a_pinned_key() {
        for shape in spr_client::SPP_SUPPORTED_SHAPES {
            let name = format!(
                "transfer_confidential_{}_{}.key",
                shape.n_inputs(),
                shape.n_outputs()
            );
            assert!(pinned_key(&name).is_ok(), "{name} is not pinned");
        }
    }

    #[test]
    fn pinned_keys_match_the_verifying_keys() {
        for (name, sha256) in spr_interface::verifying_keys::PROVING_KEY_SHA256S {
            if let Ok(key) = pinned_key(name) {
                assert_eq!(key.sha256, *sha256, "{name}");
            }
        }
    }

    #[test]
    fn the_keys_of_spends_and_merges_are_the_shapes_the_sdk_picks() {
        assert_eq!(
            needed(4, vec![2], 0).unwrap(),
            [
                "transfer_confidential_1_2.key",
                "transfer_confidential_2_2.key",
                "transfer_confidential_3_2.key",
                "transfer_confidential_4_2.key",
            ]
        );
        // Seven notes prove with the eight-input shape, three outputs with four.
        let wide = needed(7, vec![3], 25).unwrap();
        assert!(wide.contains(&"transfer_confidential_8_4.key".to_string()));
        assert!(!wide.contains(&"transfer_confidential_7_4.key".to_string()));
        assert_eq!(
            wide.iter()
                .filter(|name| name.starts_with("merge_"))
                .collect::<Vec<_>>(),
            ["merge_8_1.key", "merge_24_1.key", "merge_52_1.key"]
        );
        assert_eq!(
            needed(1, vec![17], 0).unwrap_err(),
            WalletError::ProvingKeyUnknown {
                name: "transfer_confidential_1_17.key".into()
            }
        );
        assert!(needed(0, vec![2], 0).unwrap().is_empty());
    }
}
