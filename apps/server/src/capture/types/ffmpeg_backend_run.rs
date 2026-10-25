use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, CaptureTarget, FfmpegBackend, Screen,
    codec_name_of, encoder_failed, open_screen, pick_codec_and_encoder, probe_encoder, run_ffmpeg,
};
use crate::hub::{Hub, ViewerRequest};
use tracing::{info, warn};

impl CaptureBackend for FfmpegBackend {
    async fn open_screen(&self, request: &ViewerRequest) -> Screen {
        open_screen(&self.settings, request).await
    }

    async fn run(&self, hub: &Hub, target: &CaptureTarget) -> Result<CaptureReport, CaptureError> {
        let ffmpeg_path = self.settings.ffmpeg_path.as_str();
        let (encoder, codec) = pick_codec_and_encoder(
            &target.codecs,
            |codec| self.picker_for(codec),
            |encoder, codec| probe_encoder(ffmpeg_path, encoder, codec),
        )
        .await?;
        let encoder_name = codec_name_of(encoder, codec);

        info!(
            "starting capture of output {} with {} at {} fps, {} kbps",
            target.output, encoder_name, target.fps, self.settings.video_bitrate_kbps
        );

        let result = run_ffmpeg(&self.settings, target, encoder, codec, hub).await;

        if encoder_failed(&result) {
            warn!("ffmpeg stopped before the first frame with {encoder_name}");
            self.picker_for(codec).mark_failed(encoder);
        }

        result
    }
}
