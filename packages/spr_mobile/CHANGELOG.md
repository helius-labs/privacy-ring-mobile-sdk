## 0.1.0

Unreleased.

* Built on Zolana commit `c5297afa` (SDK 0.9.0-alpha): shielded pool
  `ring1GaLCuJnkUMVkX2JeZLTTX17wKsV4fZmPVzXANW`, user registry
  `regy3WAeRXMYuMYxr5iK3Bf9Arr3PcxGxyzAddbiqDQ`, and the core ceremony's
  transfer and merge keys. The viewing key is an X25519 key. History reads
  use the indexer's paginated responses.
* `SprWallet`: registration, deposit and spends of SOL and SPL Token /
  Token-2022 assets, proving on the device. The Solana key stays with the
  application's `SolanaSigner`.
* The wallet keeps no chain state: `balances()`, `privateBalance()`,
  `activity()` and every spend read the indexer when they run.
  The wallet holds every mint the shielded pool registered, reading a mint's
  asset id and token program when a call names it or a read meets its
  notes; `WalletConfig.mints` limits it to the listed mints, each with its
  token program, and then reads only their asset ids. A mint the pool has
  not registered yet reads as no balance and no history, and its first
  deposit registers it in the same transaction:
  `Operation.deposit.createsSplInterface` shows the extra rent. A spend takes the
  largest notes first, at most 40 from at most two trees, as the Rust
  client selects them, and fails with `WalletError.mergeRequired` or
  `WalletError.tooManyInputTrees` beyond that and with
  `WalletError.amountZero` for a zero amount.
* `WalletConfig.maxSpendInputs` caps the notes a spend takes on the device
  (40 when unset); a spend that needs more fails with
  `WalletError.mergeRequired` before proving.
* `TokenBalance` reports `amount`, `reserved` included; `notes`, how many
  the amount is in; `reserved`, what this wallet's prepared spends hold; and
  `limits`, the most one spend of 2, 4, 8 or 16 outputs can take now without
  a merge, from the unreserved notes within the width's note cap and two
  trees. A spend with `k` private outputs uses `k + 1` outputs, public
  outputs take no slot.
* `WalletConfig.reuseNotesMs` lets a spend take its notes from a read made
  within that window instead of reading the indexer again; a send or a
  confirmation ends the reuse. A spend's other lookups, the recipient's
  registration or token account, run beside the read, and the wait for
  the indexer to reach the RPC's slot is asked again after a tenth of a
  second at first.
* `activity()`: the wallet's history, newest first, one row per asset, as
  the Rust client classifies it: `deposit`, `received`, `sent`, `withdrawal`
  or `selfTransfer`. Each `ActivityEntry` has a stable `index`, the pool's
  asset id; `blockTime` (Unix seconds) from bounded per-slot RPC reads,
  newest first, cached for the open wallet, `null` while unavailable and
  asked again after a growing wait; and, on outgoing rows, the private
  `recipients` read from their tags without extra RPC reads.
* `overview()`: the balances and the history from one read of the indexer,
  for a screen that shows both.
  `unresolvedAssets` lists asset ids and mints that remain unresolved after
  refreshing the pool's registrations. Their notes are excluded from balances
  and history, so non-empty lists mean the portfolio is incomplete.
* `SprWallet.transaction(signature)` fetches and decrypts one signature
  without history pagination. Its result is ready (possibly empty), pending
  indexing, not found or incomplete, with the same rows as `activity()` and
  `overview()`, the transaction's `blockTime` and network `fee` (lamports),
  and `outputs`: each outgoing private payment and public withdrawal with
  recipient, mint, amount and `isPublic`, excluding change and padding.
  Outgoing reads also fetch confirmed public settlements and recipient and
  asset metadata, and recognize cached spends. `transaction(signature,
  prepared: payment)` authenticates dummy padding from the payment's saved
  output openings, so an exact-balance payment reads without scanning wallet
  history; without them such padding returns `incomplete`.
  `TransactionLookup` compares by value.
* `spend` / `prepareSpend` take `outputs` of one asset, proved together in
  one transaction: `Output.private`, a note for a registered wallet with its
  amount hidden and the note tagged with the recipient's Solana address on
  chain, and `Output.public`, funds to a Solana account with recipient and
  amount public, tokens to its associated token account, which the spend
  creates when it does not exist, the fee payer paying its rent. The
  application's fee is one more private output to its registered collector.
  `Operation_Spend` lists the outputs in order and the recipients whose
  token account it creates. An empty list fails with `WalletError.noOutputs`.
* `PreparedTransaction.outputs`: a prepared spend's output openings,
  including change and padding, so an application's signer can recompute
  commitments and check the message. The example performs these checks
  before signing. Seeds stay inside the SDK.
* `prepareRegistration`, `prepareDeposit`, `prepareSpend`, `submit` and
  `confirm` for applications that sign and send transactions themselves.
  `feePayer` lets another account pay the network fee of a spend.
* `prepareRegistration(enableMerging: true)` registers and selects
  `PoseidonCtrV1` in the same message; without it registration does not
  turn merges on.
* Priority fees: `WalletConfig.priorityFeeLamports`, and
  `priorityFeeLamports` on every `prepare` method and on `register`,
  `deposit`, `spend`, `setMerging` and `merge`, set a total fee per transaction
  in lamports. A call overrides the configured fee, including with zero.
  `PreparedTransaction.priorityFeeLamports` reads the fee from the message
  header before signing; `refresh` preserves it.
* Merges: `setMerging` / `prepareMerging` and `mergingEnabled()` for the
  account's opt-in, and `merge` / `prepareMerge` to combine up to 52 notes of
  one asset on one tree into one, proved on the device or by `proverUrl`.
  A merge needs only the fee payer's signature and expires ten minutes after
  it is prepared.
* `spendableNotes()` lists the unreserved, nonzero notes with stable
  commitment IDs, mint, amount, tree ID and merge eligibility.
* Sponsored cache merges: `prepareMerge(cacheTarget: ...)` builds an unsigned
  CreateCache + MergeTransact, from exact ordered `inputIds` when given: it
  infers the mint and rejects missing, reserved, duplicate or incompatible
  inputs without replacing them. `verifyCachedOutputs` checks the cache
  account, and `prepareSpend(cachedInputs: ...)` consumes its verified slots
  with a local proof. The sponsor signs merges and the user signs the final
  payment. Both paths enforce the run's priority fee cap.
* `exportPreparedTransaction` saves a prepared transaction as a versioned
  snapshot authenticated with the wallet's keys; `restorePreparedTransaction`
  restores it with its cache descriptors, output openings and note
  reservations.
* `submit` looks the signature up after a send error that is not the chain's
  verdict, using RPC history and indexed events. It returns
  `WalletError.transactionOutcomeUnknown` when neither source establishes
  an outcome, including after blockhash expiry; `confirm` and
  `waitForTransaction` recover the same way.
  A spend of a note the chain already spent fails with
  `WalletError.notesAlreadySpent`.
* Balances, activity and every spend wait for the indexer to reach the RPC's
  confirmed slot, and fail with `WalletError.indexerBehind` when it does not
  within about ten seconds.
* Proving keys download on first use, pinned by `zopro-keys.lock`, in ranged
  parts of 8 MB, each written to `TransportRequest.downloadPath` as it
  arrives, so a key never sits in memory and an interrupted download resumes
  where it stopped. With the default transport, a part that receives no data
  for 30 s fails with `WalletError.provingKeyDownloadFailed`. One key
  downloads once however many calls need it. A checked key is recorded beside
  it and not hashed again until its file changes. The keys of one lockfile
  live in a directory of their own under `provingKeyDir`; opening a wallet
  removes those of other lockfiles.
* `prefetchProvingKeys(maxInputs: ..., outputs: ..., maxMergeInputs: ...)`
  downloads the keys of those spends and merges ahead of time with a
  progress stream, and stops when its subscription is cancelled;
  `provingKeys()` lists the keys and how much of each is on the device;
  `clearProvingKeys()` removes them.
* Proving keys load once per device: the first load writes the key's
  image beside it (`<key>.zopro.zopro-raw-3.solver2`: its points in the
  device's memory layout and its solver), and later loads map that file
  instead of decompressing the key, so a restart costs no key preparation
  and the system may drop the points' pages under memory pressure rather
  than the app holding them.
* `proveRequest(keyName: ..., requestJson: ...)` proves an SPR `/prove`
  request with a pinned key through the wallet's key store and reports where
  the time went: the key's download, its load, the witness, the proof and
  its verification. The example's `proof_timing_test.dart` times the
  fixture requests of 1 to 24 notes and a merge of 24 with it, on CI and
  on phones, and `scripts/proof-timing-report.py` writes the report.
* `registrationStatus()`: `notRegistered`, `registered` or `conflict`.
  `register()` never replaces a record that holds other keys.
* Prepared spends reserve their notes, so the next spend selects others;
  `release(tx)` frees them, and so does a send with a verdict: landed,
  refused by the chain, or a note already spent. A send whose
  outcome is unknown keeps them. `waitForTransaction(signature,
  lastValidBlockHeight:)` waits for a transaction sent before a restart and
  ends confirmed, refused or unknown, as `submit` does.
* `refresh(tx)`: a prepared transaction with a new blockhash and the same
  proof, for slow approvals. Every prepared transaction carries its
  `operation`, `feePayer`, `signers`, `message` and `lastValidBlockHeight`.
  `operation` has one variant per kind, each with its own fields
  (`Operation_Spend` its `mint`, `outputs` and `createdTokenAccounts`,
  `Operation_Merging` whether it turns merges on, `Operation_Custom` the
  top-level programs of a transaction the application composes, which the
  wallet does not prepare yet); the application builds its approval text
  from it, and the `SolanaSigner` receives the transaction it signs.
* `exportKeys()` and `SprWallet.openWithKeys()`: open the wallet from its
  saved derived keys, without a derivation signature; the signer is optional.
* `close()` for lock and account switch: it rejects queued operations and a
  `prepare` call that is proving at the time, never signs or submits after the
  call, and releases the native wallet once the running step ends. It aborts
  the default transport's requests in flight instead of waiting for them;
  the step they held fails with `WalletError.walletClosed`.
* Backend proving: `Proving.remote` proves a spend or merge with the Rust
  client's prover client at `WalletConfig.proverUrl` (the application's
  backend, which proxies the prover's `/prove/<key>` routes, or a prover
  directly), through the wallet's transport. The device verifies the
  returned proof against the pinned verifying key before it builds the
  message. `WalletConfig.proving` sets the default, local when unset;
  `proving` on `prepareSpend` and `spend` chooses per call.
  `WalletConfig.proverSyncOnly` refuses queued proofs and status polling.
* Remote proof requests go only to a prover that attests, at
  `/attestation`, to the application's TEE policy, and are encrypted to it
  with `Spr-Tee*` headers, the Helius gateway included; only a loopback
  prover gets plaintext, unless `requireProverTee` is set.
  `WalletConfig.proverTeeNetwork` selects the SDK-verified signed policy of
  mainnet or devnet, which the Rust client refreshes and re-attests;
  `proverTeePolicyUrl` downloads it from another HTTPS URL, such as a
  staging deployment's, verified by the same pinned signers.
  `proverTeePolicy` is a static reviewed policy for a custom deployment;
  supplying it with `proverTeeNetwork` is rejected, and a prover that must
  attest without either fails with `WalletError.proverPolicyMissing` before
  any request.
* The wallet never opens a connection: every request (Solana RPC, indexer,
  proving-key downloads, remote proofs) goes through a transport in Dart, the
  package's own on `package:http` by default, or the application's networking
  passed as `transport` to `SprWallet.open` and
  `SprWallet.openWithKeys`. With the default transport, `open` refuses
  plaintext URLs off loopback unless `allowInsecureHttp` is passed. A
  transport returns each response with its headers, and throws
  `TransportResponseLost` when a body fails after its status arrived, so a
  proof request is not sent twice.
* Errors are `WalletError`s with the data of what happened: amounts,
  accounts, mints. They carry no key material, and `api-key` values in them
  are masked.
* `LocalProver`: load a zopro key once and prove structured SPR requests
  repeatedly, one at a time; `close()` fails a proof in progress with
  `WalletError.proverClosed`. `poseidonHash`.
* `initSprMobile()` loads the native library on every platform.
* On Android the package requests `android.permission.INTERNET`, which
  Flutter's template grants only to debug and profile builds.
* Android native libraries are aligned for 16 KB memory pages.
* Signed precompiled native libraries for Android (API 24+) and iOS (15+);
  building from source with Rust is the fallback.
* iOS: a build hook bundles the native library as
  `mopro_flutter_bindings.framework`, so apps build with Swift Package
  Manager or CocoaPods.
* Examples: a devnet wallet with public demo accounts and a proof benchmark,
  and a sponsored cache-merge flow (`example/lib/main_sponsored.dart`) with
  an authenticated TEE proof proxy and sponsor relayer
  (`example/backend`) that keeps a persistent run journal. Both select the
  signed devnet policy.
