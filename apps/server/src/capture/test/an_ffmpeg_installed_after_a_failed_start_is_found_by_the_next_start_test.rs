use super::{FakeFfmpeg, avc_target, default_settings};
use crate::capture::{CaptureBackend, CaptureError, FFMPEG_FILE_NAME, FfmpegBackend};
use crate::hub::Hub;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn an_ffmpeg_installed_after_a_failed_start_is_found_by_the_next_start() {
    let installed = FakeFfmpeg::installed();
    let search_dirs = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    let backend = FfmpegBackend::with_search_dirs(default_settings(), {
        let search_dirs = search_dirs.clone();

        move || {
            search_dirs
                .lock()
                .expect("the directories are readable")
                .clone()
        }
    });
    let hub = Hub::new();

    let before = backend.run(&hub, &avc_target()).await;

    search_dirs
        .lock()
        .expect("the directories are writable")
        .push(installed.directory.clone());

    let after = backend.run(&hub, &avc_target()).await;

    assert!(matches!(before, Err(CaptureError::FfmpegNotFound { .. })));
    assert!(matches!(
        after,
        Err(CaptureError::Spawn { path, .. })
            if path == installed.directory.join(FFMPEG_FILE_NAME).display().to_string()
    ));
}
