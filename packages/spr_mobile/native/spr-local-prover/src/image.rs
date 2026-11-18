use crate::ProverError;

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

/// Where a key file's image is kept (`zopro_prover::raw`): the key's
/// points as the kernel reads them in place and its solver's image, written
/// at the key's first load and mapped at every load, so the system may drop
/// and read again the points' pages under memory pressure instead of the
/// app holding them, and a load decompresses nothing (16 -> 2 on a phone:
/// 0.4 s rather than 8.6, ~140 MB of the app's own memory rather than
/// ~360). The name carries both layouts' versions: a kernel that reads
/// another writes its own.
pub fn image_path(key: &std::path::Path) -> std::path::PathBuf {
    let mut name = key
        .file_name()
        .map(|n| n.to_os_string())
        .unwrap_or_default();
    name.push(format!(
        ".{}.solver{}",
        zopro_spr::zopro_prover::raw::FORMAT,
        zopro_spr::zopro_core::solver_image_version()
    ));
    key.with_file_name(name)
}

/// A key file loaded through its image: mapped, written from the key file
/// first when there is none or it does not load. A device that cannot
/// write it (a full disk) proves from the key file in memory.
pub(crate) fn load_key(path: &std::path::Path) -> Result<zopro_spr::Prover, ProverError> {
    device_log();
    let failed = |e: &dyn std::fmt::Display| {
        log::warn!("zopro: {}: {e}", path.display());
        ProverError::LoadFailed
    };
    // The GPU driver's compiled kernels, beside the keys: a phone compiles
    // them once rather than at every launch.
    zopro_spr::zopro_core::set_gpu_pipeline_cache(&path.with_file_name("zopro-gpu.cache"));
    let image = image_path(path);
    if image.is_file() {
        match load_image(&image) {
            Ok(prover) => return Ok(prover),
            Err(e) => log::warn!("zopro: {}: {e}; written again", image.display()),
        }
    }
    match write_image(path, &image).and_then(|()| load_image(&image)) {
        Ok(prover) => Ok(prover),
        Err(e) => {
            log::warn!("zopro: {}: {e}; proving from the key file", image.display());
            let _ = std::fs::remove_file(&image);
            let key = map(path).map_err(|e| failed(&e))?;
            zopro_spr::Prover::load(&key).map_err(|e| failed(&e))
        }
    }
}

/// `path` mapped read-only. The store writes keys and images beside and
/// renames them into place, so a mapped file is never written.
fn map(path: &std::path::Path) -> Result<memmap2::Mmap, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    // SAFETY: nothing writes or truncates the file in place (see above).
    unsafe { memmap2::Mmap::map(&file) }.map_err(|e| e.to_string())
}

fn load_image(image: &std::path::Path) -> Result<zopro_spr::Prover, String> {
    let prover = zopro_spr::zopro_prover::Prover::load_image(std::sync::Arc::new(map(image)?))?;
    zopro_spr::Prover::new(prover)
}

/// The key's image, whole or absent: written beside, then renamed.
fn write_image(key: &std::path::Path, image: &std::path::Path) -> Result<(), String> {
    let key = map(key)?;
    let partial = image.with_extension("partial");
    let written = (|| -> Result<(), String> {
        let file = std::fs::File::create(&partial).map_err(|e| e.to_string())?;
        let mut out = std::io::BufWriter::new(file);
        zopro_spr::zopro_prover::Prover::write_image(&key, &mut out)?;
        let file = out.into_inner().map_err(|e| e.to_string())?;
        file.sync_all().map_err(|e| e.to_string())?;
        std::fs::rename(&partial, image).map_err(|e| e.to_string())
    })();
    if written.is_err() {
        let _ = std::fs::remove_file(&partial);
    }
    written
}

#[cfg(test)]
mod tests {
    use super::image_path;
    use std::path::{Path, PathBuf};

    #[test]
    fn image_name_includes_both_layout_versions() {
        assert_eq!(
            image_path(Path::new("keys/k.key")),
            PathBuf::from("keys").join(format!(
                "k.key.{}.solver{}",
                zopro_spr::zopro_prover::raw::FORMAT,
                zopro_spr::zopro_core::solver_image_version()
            ))
        );
    }
}
