use super::ScriptedProbe;
use crate::capture::{CaptureError, VideoEncoder};

impl ScriptedProbe {
    pub async fn probe(&self, encoder: VideoEncoder) -> Result<bool, CaptureError> {
        self.calls
            .lock()
            .expect("the calls lock is not poisoned")
            .push(encoder);

        if self.missing_ffmpeg {
            return Err(CaptureError::FfmpegNotFound {
                path: "ffmpeg".to_owned(),
            });
        }

        Ok(self.passing.contains(&encoder))
    }
}
