//! Where the wallet proves a spend: on the device with its own [`Prover`], or
//! with the Zolana SDK's prover client through the application's transport.

use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use spr_client::{prover::ExpectedProvingKey, ClientError, Proof, ProveRequest, Prover};

use crate::{elapsed_ms, error::WalletError, keys, Loaded, ProvedRequest, PREPARED};

mod input;

pub(crate) use zopro_spr::zopro_prover::disk::image_path;

/// Where a spend is proved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proving {
    /// On the device, with the pinned proving key for its shape. The witness
    /// stays in this process.
    Local,
    /// By the prover at [`WalletConfig::prover_url`](crate::WalletConfig::prover_url),
    /// through the application's transport. The prover receives the witness,
    /// the wallet's nullifier secret included.
    Remote,
}

/// Proves the client's assembled inputs with the named key, loading that
/// key into the shared slot when it changes. Clones share one prover.
#[derive(Clone)]
pub(crate) struct NativeProver(Arc<Native>);

struct Native {
    keys: Arc<keys::KeyStore>,
    /// Id of the key this prover loaded last; 0 before the first.
    loaded: AtomicU64,
}

/// The prover's log lines into the device log: warnings (a GPU proof that
/// failed to verify), and with the zopro-trace feature the stage timings
/// the Device Farm bench reads.
fn device_log() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        #[cfg(target_os = "ios")]
        let _ = oslog::OsLogger::new("zolana.zopro")
            .level_filter(if cfg!(feature = "zopro-trace") {
                log::LevelFilter::Info
            } else {
                log::LevelFilter::Warn
            })
            .init();
        #[cfg(target_os = "android")]
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(if cfg!(feature = "zopro-trace") {
                    log::LevelFilter::Info
                } else {
                    log::LevelFilter::Warn
                })
                .with_tag("zopro"),
        );
    });
}

/// Load a trusted key through zopro's disk cache.
pub(crate) fn load_key(path: &std::path::Path) -> Result<zopro_spr::Prover, WalletError> {
    device_log();
    zopro_spr::Prover::load_cached(path).map_err(|error| {
        log::warn!("zopro: {}: {error}", path.display());
        WalletError::ProverLoadFailed
    })
}

/// Why a proof on the device failed, carried through the client as
/// [`ClientError::ProverFailed`] and read back as the [`WalletError`] it is.
#[derive(Debug)]
pub(crate) struct DeviceProverFailure(pub(crate) WalletError);

impl std::fmt::Display for DeviceProverFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.0)
    }
}

impl std::error::Error for DeviceProverFailure {}

impl NativeProver {
    pub(crate) fn new(keys: Arc<keys::KeyStore>) -> Self {
        Self(Arc::new(Native {
            keys,
            loaded: AtomicU64::new(0),
        }))
    }

    /// Loads a key in place of the loaded one (a phone has room for one).
    fn load(
        &self,
        state: &mut crate::ProverState,
        name: &str,
        path: &std::path::Path,
    ) -> Result<(), WalletError> {
        state.loaded = None;
        let prover = load_key(path)?;
        let id = state.next_id()?;
        self.0.loaded.store(id, Ordering::Relaxed);
        state.loaded = Some(Loaded {
            id,
            key: Some(name.to_string()),
            prover,
        });
        Ok(())
    }

    /// Prove `body` with `key`, fetched and loaded as needed, and say where
    /// the time went. The proof is verified before it leaves.
    pub(crate) fn prove_request(
        &self,
        body: &str,
        key: &ExpectedProvingKey,
    ) -> Result<ProvedRequest, WalletError> {
        self.prove_with(key, |prover| prover.prove(body))
    }

    fn prove_with(
        &self,
        key: &ExpectedProvingKey,
        prove: impl FnOnce(&mut zopro_spr::Prover) -> Result<zopro_spr::ProofResult, zopro_spr::Error>,
    ) -> Result<ProvedRequest, WalletError> {
        let started = Instant::now();
        let name = key.name.as_str();
        let (path, downloaded_bytes) = self.0.keys.ensure(key)?;
        let download_ms = elapsed_ms(started);
        let mut state = PREPARED
            .lock()
            .map_err(|_| WalletError::ProverUnavailable)?;
        let loading = Instant::now();
        let loaded_key = state.loaded.as_ref().map(|loaded| loaded.key.as_deref());
        if loaded_key != Some(Some(name)) {
            if loaded_key == Some(None) {
                // The application holds its own key; leave it.
                return Err(WalletError::ProverBusy);
            }
            self.load(&mut state, name, &path)?;
        }
        let load_ms = elapsed_ms(loading);
        let loaded = state.loaded.as_mut().ok_or(WalletError::ProverClosed)?;
        let proof = prove(&mut loaded.prover).map_err(|error| match error {
            zopro_spr::Error::Unsupported => WalletError::UnsupportedCircuit,
            zopro_spr::Error::Prove(message) => {
                log::warn!("zopro: {name}: {message}");
                WalletError::ProofFailed
            }
            _ => WalletError::ProofFailed,
        })?;
        Ok(ProvedRequest {
            proof_json: proof.proof_json.to_string(),
            key: name.to_string(),
            downloaded_bytes,
            download_ms,
            load_ms,
            witness_ms: proof.witness_ms,
            prove_ms: proof.prove_ms,
            verify_ms: proof.verify_ms,
            total_ms: elapsed_ms(started),
        })
    }
}

/// A closed wallet releases the proving key it loaded, unless another prover
/// has replaced it since.
impl Drop for Native {
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

/// Proves in the calling thread, so [`ProveRequest::delivery`] does not apply.
impl Prover for NativeProver {
    fn prove(&self, request: &dyn ProveRequest) -> Result<Proof, ClientError> {
        let key = request.proving_key()?;
        let prove = || {
            let inputs = request
                .local_inputs()
                .ok_or(WalletError::UnsupportedCircuit)?;
            let input = input::convert(inputs)?;
            self.prove_with(&key, |prover| prover.prove_input(&input))
        };
        let proof = prove()
            .map_err(|failure| ClientError::ProverFailed(Box::new(DeviceProverFailure(failure))))?;
        Proof::from_gnark_json(&proof.proof_json)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        sync::atomic::{AtomicU64, Ordering},
    };

    use solana_address::Address;
    use spr_client::{
        assemble,
        prover::{verify_proof_statement, LocalProvingInputs},
        AssembledTransfer, Delivery, ProverClient,
    };
    use spr_interface::{verifying_keys::CircuitId, N_PUBLIC_SLOTS};
    use spr_keypair::ShieldedKeypair;
    use spr_transaction::instructions::transact::{ConfidentialTransaction, Shape};
    use zeroize::Zeroizing;

    use super::*;
    use crate::fixture_trees::Trees;
    use crate::transport::{
        tests::{fake, ok, Requests},
        TransportFailure, TransportRequest, TransportResponse,
    };

    const REQUEST: &str = include_str!("../../../../../fixtures/prove-request-2x2.json");
    /// A proof of [`REQUEST`] with the pinned key, as a prover returns it.
    const RESPONSE: &[u8] = include_bytes!("../../../../../fixtures/prove-response-2x2.json");
    /// A loopback prover, the one kind the SDK sends plaintext requests to.
    const PROVER_URL: &str = "http://prover.localhost/v1/spr?api-key=secret";
    const PROVE_URL: &str =
        "http://prover.localhost/v1/spr/prove/transfer_confidential_2_2?api-key=secret";
    const STATUS_URL: &str = "http://prover.localhost/v1/spr/prove/transfer_confidential_2_2/status?api-key=secret&jobId=job-1";

    /// The SDK's prover client of [`PROVER_URL`], through a transport that
    /// answers each request with `answer`.
    fn remote(
        answer: impl Fn(&TransportRequest) -> Result<TransportResponse, TransportFailure>
            + Send
            + Sync
            + 'static,
    ) -> (ProverClient, Requests) {
        let (transport, requests) = fake(answer);
        (
            transport
                .configured_prover(PROVER_URL.to_string(), false, None, None, None, false)
                .unwrap(),
            requests,
        )
    }

    fn sent(requests: &Requests) -> Vec<(String, String)> {
        requests
            .lock()
            .unwrap()
            .iter()
            .map(|request| (request.method.clone(), request.url.clone()))
            .collect()
    }

    /// `proof` is a proof of [`REQUEST`], the fixture's own.
    fn assert_proves_request(proof: &Proof) {
        let request: serde_json::Value = serde_json::from_str(REQUEST).unwrap();
        let public_input = request["publicInputHash"].as_str().unwrap();
        let public_input = format!("{:0>64}", public_input.trim_start_matches("0x"));
        let public_input: Vec<u8> = (0..64)
            .step_by(2)
            .map(|i| u8::from_str_radix(&public_input[i..i + 2], 16).unwrap())
            .collect();
        let circuit = CircuitId::ConfidentialSigner(2, 2, N_PUBLIC_SLOTS as u8);
        verify_proof_statement(
            proof,
            public_input.try_into().unwrap(),
            circuit.verifying_key().unwrap(),
        )
        .expect("the proof verifies");
    }

    #[test]
    fn a_remote_proof_comes_in_the_response() {
        let (prover, requests) = remote(|_| {
            Ok(TransportResponse {
                status: 200,
                headers: Default::default(),
                body: RESPONSE.to_vec(),
            })
        });
        let proof = prover
            .prove(&Captured(transfer_2_2_key()))
            .expect("the prover's proof");
        assert_proves_request(&proof);

        let request = requests.lock().unwrap().pop().unwrap();
        assert_eq!(
            (request.method.as_str(), request.url.as_str()),
            ("POST", PROVE_URL)
        );
        let headers = HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            ("x-sync".to_string(), "true".to_string()),
        ]);
        assert_eq!(request.headers, headers);
        assert_eq!(request.body, REQUEST.as_bytes());
        // A synchronous proof is not cut at the transport's 30 s.
        assert_eq!(
            (request.max_response_bytes, request.timeout_ms),
            (None, Some(599_000))
        );
        assert!(requests.lock().unwrap().is_empty());
    }

    #[test]
    fn a_queued_remote_proof_is_polled_until_it_completes() {
        let polls = AtomicU64::new(0);
        let completed = format!(
            r#"{{"jobId":"job-1","status":"completed","result":{}}}"#,
            std::str::from_utf8(RESPONSE).unwrap()
        );
        let (prover, requests) = remote(move |request| match request.method.as_str() {
            "POST" => ok(r#"{"jobId":"job-1","status":"queued"}"#),
            _ if polls.fetch_add(1, Ordering::Relaxed) == 0 => {
                ok(r#"{"jobId":"job-1","status":"processing"}"#)
            }
            _ => ok(&completed),
        });
        let proof = prover
            .prove(&Captured(transfer_2_2_key()))
            .expect("the queued proof");
        assert_proves_request(&proof);

        let get = |url: &str| ("GET".to_string(), url.to_string());
        assert_eq!(
            sent(&requests),
            [
                ("POST".to_string(), PROVE_URL.to_string()),
                get(STATUS_URL),
                get(STATUS_URL)
            ]
        );
        for poll in &requests.lock().unwrap()[1..] {
            assert!(poll.headers.is_empty() && poll.body.is_empty());
            assert_eq!(poll.timeout_ms, Some(29_000));
        }
    }

    #[test]
    fn queued_failure_and_timeout_stop_without_resubmitting() {
        for fails in [true, false] {
            let (prover, requests) = remote(move |request| match request.method.as_str() {
                "POST" => ok(r#"{"jobId":"job-1","status":"queued"}"#),
                _ if fails => ok(r#"{"status":"failed","error":"proof rejected"}"#),
                _ => ok(r#"{"status":"processing"}"#),
            });
            let prover = prover.with_async_poll_config(spr_client::prover::AsyncPollConfig {
                poll_interval_secs: 1,
                max_wait_secs: if fails { 5 } else { 0 },
            });
            let error = prover
                .prove(&Captured(transfer_2_2_key()))
                .unwrap_err()
                .to_string();
            assert!(
                if fails {
                    error.contains("failed")
                } else {
                    error.contains("timed out")
                },
                "{error}"
            );
            let requests = requests.lock().unwrap();
            assert_eq!(requests.iter().filter(|r| r.method == "POST").count(), 1);
            assert!(requests.iter().any(|r| r.method == "GET"));
        }
    }

    /// A proof response whose body is still arriving at the SDK's deadline:
    /// the transport gives up a second earlier, with the status, so the SDK
    /// does not send the request again.
    #[test]
    fn a_slow_body_at_the_deadline_is_not_sent_again() {
        let (transport, requests) = fake(|request| {
            // The status arrived; the body drips until the transport's own
            // deadline, as the default transport counts it.
            let deadline = request.timeout_ms.expect("a proof request has a deadline");
            std::thread::sleep(std::time::Duration::from_millis(deadline.into()));
            Err(TransportFailure {
                message: "TimeoutException: body".to_string(),
                status: Some(200),
            })
        });
        let prover = transport
            .prover(PROVER_URL.to_string())
            .with_proof_timeout(std::time::Duration::from_secs(2));
        assert!(prover.prove(&Captured(transfer_2_2_key())).is_err());
        assert_eq!(
            sent(&requests),
            [("POST".to_string(), PROVE_URL.to_string())]
        );
    }

    #[test]
    fn a_proof_request_whose_response_is_lost_is_not_sent_again() {
        let (prover, requests) = remote(|_| {
            Err(TransportFailure {
                message: "connection reset".to_string(),
                status: Some(200),
            })
        });
        let error = WalletError::from(prover.prove(&Captured(transfer_2_2_key())).unwrap_err());
        let WalletError::Client { message: error } = error else {
            panic!("{error:?}");
        };
        assert!(
            error.contains("transport failed after status 200: connection reset"),
            "{error}"
        );
        assert!(!error.contains("secret"), "{error}");
        // The prover may be proving it: one POST, never a second.
        assert_eq!(
            sent(&requests),
            [("POST".to_string(), PROVE_URL.to_string())]
        );
    }

    /// The client verifies a remote proof in `AssembledTransfer::prove`, which
    /// `finish_submission_unsigned_sync_with_prover` calls before it fetches a
    /// blockhash and builds the message.
    #[test]
    fn a_remote_proof_that_does_not_verify_fails_the_spend() {
        let sender = ShieldedKeypair::new_ed25519().unwrap();
        let mut assembled = assembled_transfer(&sender);
        let shape = (
            assembled.prover_inputs.inputs.len(),
            assembled.prover_inputs.outputs.len(),
        );
        assert_eq!(shape, (2, 2), "the shape of the fixture proof");
        let mut unparsable: serde_json::Value = serde_json::from_slice(RESPONSE).unwrap();
        unparsable["proof"]["ar"] = serde_json::json!(["0x1"]);

        for (response, failure) in [
            // A real proof, of another transaction.
            (RESPONSE.to_vec(), WalletError::ProofInvalid),
            (b"not json".to_vec(), WalletError::ProofMalformed),
            (
                unparsable.to_string().into_bytes(),
                WalletError::ProofMalformed,
            ),
            // No `provingKeySha256`: the key it came from is unknown.
            (
                br#"{"proof":{"ar":["0x1"]}}"#.to_vec(),
                WalletError::ProofMalformed,
            ),
        ] {
            let (prover, requests) = remote(move |_| {
                Ok(TransportResponse {
                    status: 200,
                    headers: Default::default(),
                    body: response.clone(),
                })
            });
            let error = WalletError::from(assembled.prove(&prover, &sender).unwrap_err());
            assert_eq!(error, failure);
            let request = requests.lock().unwrap().pop().unwrap();
            assert_eq!(request.url, PROVE_URL);
            assert_eq!(request.headers["x-sync"], "true");
            assert_eq!(request.timeout_ms, Some(599_000));
        }
    }

    /// A SOL spend assembled against real Merkle paths from test trees.
    fn assembled_transfer(sender: &ShieldedKeypair) -> AssembledTransfer {
        let mut trees = Trees::new();
        let notes = trees.notes(sender, 1);
        let payer = Address::new_from_array(sender.signing_pubkey().as_ed25519().unwrap());
        let recipient = ShieldedKeypair::new_ed25519().unwrap();
        let mut transaction = ConfidentialTransaction::new(notes.clone(), payer).unwrap();
        transaction
            .transfer_sol(&recipient.shielded_address().unwrap(), 4)
            .unwrap();
        transaction
            .pad_utxos(Shape::IN2_OUT2, &sender.shielded_address().unwrap())
            .unwrap();
        let proof_inputs = transaction.encrypt(sender).unwrap();
        let spend = trees.spend_proofs(&notes);
        let dummies: Vec<_> = proof_inputs
            .dummy_nullifiers()
            .into_iter()
            .map(|nullifier| trees.non_inclusion(nullifier))
            .collect();
        assemble(proof_inputs, &spend, &dummies).unwrap()
    }

    /// The captured 2→2 client body, with the key it asks for, asked for in
    /// the response as the SDK asks for a transfer proof.
    struct Captured(ExpectedProvingKey);

    impl ProveRequest for Captured {
        fn body(&self) -> Result<Zeroizing<String>, ClientError> {
            Ok(Zeroizing::new(REQUEST.to_string()))
        }

        fn proving_key(&self) -> Result<ExpectedProvingKey, ClientError> {
            Ok(self.0.clone())
        }

        fn delivery(&self) -> Delivery {
            Delivery::InResponse
        }
    }

    /// A proof on the device that fails reaches the wallet as what happened,
    /// through the client's typed prover failure: here the key cannot be
    /// downloaded.
    #[test]
    fn a_device_failure_reaches_the_wallet_as_itself() {
        let dir =
            std::env::temp_dir().join(format!("zolana-mobile-unreachable-{}", std::process::id()));
        let prover = NativeProver::new(Arc::new(keys::KeyStore::new(
            dir.display().to_string(),
            None,
            crate::transport::tests::unreachable(),
        )));
        let sender = ShieldedKeypair::new_ed25519().unwrap();
        let failure = assembled_transfer(&sender)
            .prove(&prover, &sender)
            .unwrap_err();
        assert!(
            matches!(failure, ClientError::ProverFailed(_)),
            "{failure:?}"
        );
        assert_eq!(
            WalletError::from(failure),
            WalletError::ProvingKeyDownloadFailed {
                name: "transfer_confidential_2_2.key".to_string()
            }
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    /// The local backend may only read the key identity and assembled inputs.
    struct TypedOnly<'a>(&'a dyn ProveRequest);

    impl ProveRequest for TypedOnly<'_> {
        fn body(&self) -> Result<Zeroizing<String>, ClientError> {
            panic!("local proving must not serialize a wire body");
        }

        fn local_inputs(&self) -> Option<LocalProvingInputs<'_>> {
            self.0.local_inputs()
        }

        fn proving_key(&self) -> Result<ExpectedProvingKey, ClientError> {
            self.0.proving_key()
        }
    }

    struct RequireTyped<'a>(&'a NativeProver);

    impl Prover for RequireTyped<'_> {
        fn prove(&self, request: &dyn ProveRequest) -> Result<Proof, ClientError> {
            self.0.prove(&TypedOnly(request))
        }
    }

    /// Uses the pinned transfer key staged in `ZOLANA_TEST_KEY_DIR`.
    #[test]
    #[ignore = "needs the pinned transfer key in ZOLANA_TEST_KEY_DIR"]
    fn proves_a_client_request_with_the_pinned_key() {
        let prover = native_with_staged_keys();
        let sender = ShieldedKeypair::new_ed25519().unwrap();
        let mut transfer = assembled_transfer(&sender);
        let key = transfer_2_2_key();
        let path = prover.0.keys.ensure(&key).unwrap().0;
        let standalone = crate::LocalProver::load(path.display().to_string()).unwrap();
        let failure = transfer.prove(&RequireTyped(&prover), &sender).unwrap_err();
        assert_eq!(WalletError::from(failure), WalletError::ProverBusy);
        standalone.close();

        let image = image_path(&path);
        let modified = std::fs::metadata(&image).unwrap().modified().unwrap();
        let reopened = crate::LocalProver::load(path.display().to_string()).unwrap();
        assert_eq!(
            std::fs::metadata(&image).unwrap().modified().unwrap(),
            modified
        );
        reopened.close();

        transfer
            .prove(&RequireTyped(&prover), &sender)
            .expect("prove and verify the transaction with typed inputs");
        assert!(image_path(&path).is_file());
        // The same loaded key serves the next proof.
        transfer.prove(&RequireTyped(&prover), &sender).unwrap();
    }

    #[test]
    #[ignore = "needs the pinned merge_8_1.zopro key in ZOLANA_TEST_KEY_DIR"]
    fn proves_a_typed_merge_with_the_pinned_key() {
        use spr_client::{MergeProver, ProverExt};
        use spr_interface::verifying_keys::merge_8_1::VERIFYINGKEY;
        use spr_transaction::instructions::merge::MergeTransaction;

        let prover = native_with_staged_keys();
        let owner = ShieldedKeypair::new_ed25519().unwrap();
        let mut trees = Trees::new();
        let notes = trees.notes(&owner, 2);
        let transaction = MergeTransaction::new(notes.clone())
            .unwrap()
            .with_output_tree_id(crate::fixture_trees::TREE_ID)
            .with_expiry(u64::MAX)
            .encrypt(&owner)
            .unwrap();
        let dummy_nullifier_proofs = transaction
            .dummy_nullifiers()
            .into_iter()
            .map(|nullifier| trees.non_inclusion(nullifier))
            .collect();
        let result = MergeProver {
            transaction,
            nullifier_key: owner.nullifier_key.clone(),
            proofs: trees.spend_proofs(&notes),
            dummy_nullifier_proofs,
            cache: None,
        }
        .build()
        .unwrap();
        assert_eq!(result.inputs.inputs.len(), 8);
        let proof = RequireTyped(&prover).prove_merge(&result.inputs).unwrap();
        verify_proof_statement(&proof, result.public_input_hash, &VERIFYINGKEY).unwrap();
    }

    fn native_with_staged_keys() -> NativeProver {
        let dir = std::env::var("ZOLANA_TEST_KEY_DIR").unwrap_or_else(|_| {
            std::env::temp_dir()
                .join("zolana-mobile-keys")
                .display()
                .to_string()
        });
        NativeProver::new(Arc::new(keys::KeyStore::new(
            dir,
            None,
            crate::transport::tests::unreachable(),
        )))
    }

    /// Proves every `fixtures/prove-request-<n>x<m>.json` with its pinned
    /// key through [`crate::ProvingKeys::prove`]. The keys must be in
    /// `ZOLANA_TEST_KEY_DIR` (or a temp directory) already, the widest of
    /// them 180 MB: this transport downloads nothing.
    #[test]
    #[ignore = "needs the proving keys of every fixture"]
    fn proves_every_fixture_with_its_pinned_key() {
        let dir = std::env::var("ZOLANA_TEST_KEY_DIR").unwrap_or_else(|_| {
            std::env::temp_dir()
                .join("zolana-mobile-keys")
                .display()
                .to_string()
        });
        let store = Arc::new(keys::KeyStore::new(
            dir,
            None,
            crate::transport::tests::unreachable(),
        ));
        let keys = crate::ProvingKeys {
            prover: NativeProver::new(Arc::clone(&store)),
            store,
        };
        let fixtures =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../fixtures");
        let mut proved = 0;
        for entry in std::fs::read_dir(&fixtures).unwrap() {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_str().unwrap();
            let Some(shape) = name
                .strip_prefix("prove-request-")
                .and_then(|rest| rest.strip_suffix(".json"))
            else {
                continue;
            };
            let request = std::fs::read_to_string(&path).unwrap();
            let key = match shape.strip_prefix("merge-") {
                Some(merge) => format!("merge_{}.key", merge.replace('x', "_")),
                None => format!("transfer_confidential_{}.key", shape.replace('x', "_")),
            };
            let result = keys
                .prove(key.clone(), request)
                .unwrap_or_else(|failure| panic!("{name} with {key}: {failure:?}"));
            assert!(!result.proof_json.is_empty() && result.key == key, "{name}");
            // The client checks this against its pins.
            let proof: serde_json::Value = serde_json::from_str(&result.proof_json).unwrap();
            assert_eq!(
                proof["provingKeySha256"].as_str(),
                keys::key_sha256(&key),
                "{name}"
            );
            println!(
                "{name}: download {} ms ({} bytes), load {} ms, witness {} ms, prove {} ms, verify {} ms",
                result.download_ms, result.downloaded_bytes, result.load_ms,
                result.witness_ms, result.prove_ms, result.verify_ms
            );
            proved += 1;
        }
        assert!(proved >= 4, "{proved} fixtures proved");
    }

    fn transfer_2_2_key() -> ExpectedProvingKey {
        let name = "transfer_confidential_2_2.key";
        let (_, sha256) = spr_interface::verifying_keys::PROVING_KEY_SHA256S
            .iter()
            .find(|(file, _)| *file == name)
            .expect("pinned 2x2 key");
        ExpectedProvingKey {
            name: name.to_string(),
            sha256: *sha256,
        }
    }
}
