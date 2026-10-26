use super::{FakeFfmpeg, default_settings};
use crate::capture::{
    CaptureBackend, CaptureError, FFMPEG_FILE_NAME, FfmpegBackend, locate_ffmpeg,
};
use crate::hub::ViewerRequest;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn an_ffmpeg_installed_while_the_server_runs_is_found_by_the_next_preparation() {
    let installed = FakeFfmpeg::installed();
    let search_dirs = Arc::new(Mutex::new(Vec::<PathBuf>::new()));
    let backend = FfmpegBackend::with_locator(default_settings(), {
        let search_dirs = search_dirs.clone();

        move || locate_ffmpeg(&search_dirs.lock().expect("the directories are readable"))
    });
    let request = ViewerRequest::default();

    let before = backend.prepare(&request).await;

    search_dirs
        .lock()
        .expect("the directories are writable")
        .push(installed.directory.clone());

    let after = backend.prepare(&request).await;

    assert!(matches!(before, Err(CaptureError::FfmpegNotFound { .. })));
    assert!(matches!(
        after,
        Err(CaptureError::Spawn { path, .. })
            if path == installed.directory.join(FFMPEG_FILE_NAME).display().to_string()
    ));
}
