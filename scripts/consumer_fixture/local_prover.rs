use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read, Seek, SeekFrom},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use spr_local_prover::{pinned_key, KeyDownload, KeyDownloadResponse, KeyStore, ManagedProver};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args_os().skip(1);
    let request_json = fs::read_to_string(args.next().expect("fixture request"))?;
    let artifact = args.next().expect("staged zopro key");
    let cache = args.next().expect("cache directory");
    let artifact_size = fs::metadata(&artifact)?.len();

    // Serve the staged artifact like a ranged HTTP host, writing each part
    // to the transport's destination file.
    let requests = Arc::new(AtomicUsize::new(0));
    let calls = Arc::clone(&requests);
    let transport = Arc::new(move |request: KeyDownload| {
        calls.fetch_add(1, Ordering::Relaxed);
        let mut source = fs::File::open(&artifact)?;
        let (start, end) = request.range.into_inner();
        source.seek(SeekFrom::Start(start))?;
        let mut destination = fs::File::create(request.destination)?;
        let copied = io::copy(&mut source.take(end - start + 1), &mut destination)?;
        assert_eq!(copied, end - start + 1);
        Ok(KeyDownloadResponse {
            status: 206,
            headers: HashMap::from([(
                "content-range".into(),
                format!("bytes {start}-{end}/{artifact_size}"),
            )]),
            body: Vec::new(),
        })
    });

    let keys = Arc::new(KeyStore::new(cache, None, transport));
    let prover = ManagedProver::new(Arc::clone(&keys));
    let key = pinned_key("transfer_confidential_2_2.key")?;
    let proof = prover.prove(&request_json, &key)?;
    assert_eq!(proof.key, key.name);
    assert_eq!(proof.downloaded_bytes, artifact_size);
    let expected_digest: String = key
        .sha256
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    assert!(proof.proof_json.contains(&expected_digest));

    let download_calls = requests.load(Ordering::Relaxed);
    assert!(download_calls > 0);
    let (path, downloaded) = keys.ensure(&key)?;
    assert_eq!(downloaded, 0);
    assert_eq!(requests.load(Ordering::Relaxed), download_calls);
    let file = path.file_name().unwrap().to_string_lossy();
    assert!(fs::read_dir(path.parent().unwrap())?.any(|entry| {
        entry.is_ok_and(|entry| {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.starts_with(file.as_ref()) && name.contains(".solver")
        })
    }));
    println!("Verified a 2→2 proof using the standalone Rust SDK.");
    Ok(())
}
