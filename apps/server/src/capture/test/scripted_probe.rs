use crate::capture::VideoEncoder;
use std::sync::{Arc, Mutex};

pub struct ScriptedProbe {
    pub(super) passing: Vec<VideoEncoder>,
    pub(super) missing_ffmpeg: bool,
    pub(super) calls: Arc<Mutex<Vec<VideoEncoder>>>,
}

impl ScriptedProbe {
    pub fn passing(passing: &[VideoEncoder]) -> Self {
        Self {
            passing: passing.to_vec(),
            missing_ffmpeg: false,
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn without_ffmpeg() -> Self {
        Self {
            passing: Vec::new(),
            missing_ffmpeg: true,
            calls: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
