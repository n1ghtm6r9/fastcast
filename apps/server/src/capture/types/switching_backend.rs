use crate::capture::CaptureBackend;
use crate::hub::ViewerRequest;
use std::sync::{Arc, Mutex};

pub struct SwitchingBackend<N, F> {
    pub(crate) native: Arc<N>,
    pub(crate) ffmpeg: Arc<F>,
    pub(crate) native_failed_for: Mutex<Option<ViewerRequest>>,
}

impl<N: CaptureBackend, F: CaptureBackend> SwitchingBackend<N, F> {
    pub fn new(native: Arc<N>, ffmpeg: Arc<F>) -> Self {
        Self {
            native,
            ffmpeg,
            native_failed_for: Mutex::new(None),
        }
    }
}
