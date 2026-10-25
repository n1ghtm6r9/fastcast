use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, CaptureTarget, FfmpegBackend, Screen,
    codec_name_of, ended_before_the_first_frame, locate_ffmpeg, open_screen,
    pick_codec_and_encoder, probe_encoder, run_ffmpeg,
};
use crate::hub::{Hub, ViewerRequest};
use tracing::{info, warn};

impl CaptureBackend for FfmpegBackend {
    async fn open_screen(&self, request: &ViewerRequest) -> Screen {
        open_screen(&self.settings, request).await
    }

    async fn run(&self, hub: &Hub, target: &CaptureTarget) -> Result<CaptureReport, CaptureError> {
        let ffmpeg_path = locate_ffmpeg(&self.settings.ffmpeg_path, &(self.search_dirs)())?;
        let (encoder, codec) = pick_codec_and_encoder(
            &target.codecs,
            |codec| self.picker_for(codec),
            |encoder, codec| probe_encoder(&ffmpeg_path, encoder, codec),
        )
        .await?;
        let encoder_name = codec_name_of(encoder, codec);

        info!(
            "starting capture of output {} with {} using {} at {} fps, {} kbps",
            target.output, encoder_name, ffmpeg_path, target.fps, self.settings.video_bitrate_kbps
        );

        let result = run_ffmpeg(&ffmpeg_path, &self.settings, target, encoder, codec, hub).await;

        if ended_before_the_first_frame(&result) {
            warn!(
                "ffmpeg stopped before the first frame with {encoder_name}, the encoder is checked again before the next start"
            );
            self.picker_for(codec).forget_resolved();
        }

        result
    }
}
