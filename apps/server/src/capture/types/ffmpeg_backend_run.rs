use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, FfmpegBackend, encoder_failed, pick_encoder,
    probe_encoder, run_ffmpeg,
};
use crate::hub::Hub;
use tracing::{info, warn};

impl CaptureBackend for FfmpegBackend {
    async fn run(&self, hub: &Hub) -> Result<CaptureReport, CaptureError> {
        let ffmpeg_path = self.settings.ffmpeg_path.as_str();
        let encoder =
            pick_encoder(&self.picker, |encoder| probe_encoder(ffmpeg_path, encoder)).await?;

        info!(
            "starting capture of output {} with the {} encoder at {} fps, {} kbps",
            self.settings.capture_output,
            encoder,
            self.settings.capture_fps,
            self.settings.video_bitrate_kbps
        );

        let result = run_ffmpeg(&self.settings, encoder, hub).await;

        if encoder_failed(&result) {
            warn!("ffmpeg stopped before the first frame with the {encoder} encoder");
            self.picker.mark_failed(encoder);
        }

        result
    }
}
