use crate::capture::CaptureBackend;
use crate::hub::ViewerRequest;
use crate::settings::BackendChoice;
use std::sync::{Arc, Mutex};

pub struct SwitchingBackend<N, F> {
    pub(crate) choice: BackendChoice,
    pub(crate) native: Arc<N>,
    pub(crate) ffmpeg: Arc<F>,
    pub(crate) native_failed_for: Mutex<Option<ViewerRequest>>,
}

impl<N: CaptureBackend, F: CaptureBackend> SwitchingBackend<N, F> {
    pub fn new(choice: BackendChoice, native: Arc<N>, ffmpeg: Arc<F>) -> Self {
        Self {
            choice,
            native,
            ffmpeg,
            native_failed_for: Mutex::new(None),
        }
    }
}
