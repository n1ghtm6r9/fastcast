use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, CaptureTarget, FfmpegBackend, Pipeline, Screen,
    codec_name_of, codecs_to_try, ended_before_the_first_frame, monitor_refresh_hz, open_screen,
    pick_codec_and_encoder, probe_encoder, run_ffmpeg,
};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use tracing::{info, warn};

impl CaptureBackend for FfmpegBackend {
    type Pipeline = Pipeline;

    async fn prepare(&self, request: &ViewerRequest) -> Result<Pipeline, CaptureError> {
        let ffmpeg_path = (self.locate)()?;
        let (encoder, codec) = pick_codec_and_encoder(
            &codecs_to_try(self.settings.video_codec, &request.codecs),
            |codec| self.picker_for(codec),
            |encoder, codec| probe_encoder(&ffmpeg_path, encoder, codec),
        )
        .await?;

        Ok(Pipeline {
            ffmpeg_path,
            encoder,
            codec,
        })
    }

    async fn open_screen(&self, request: &ViewerRequest) -> Screen {
        open_screen(&self.settings, request, monitor_refresh_hz).await
    }

    async fn run(
        &self,
        hub: &Arc<Hub>,
        pipeline: &Pipeline,
        target: &CaptureTarget,
    ) -> Result<CaptureReport, CaptureError> {
        let encoder_name = codec_name_of(pipeline.encoder, pipeline.codec);

        info!(
            "starting capture of output {} with {} using {} at {} fps, {} kbps",
            target.output,
            encoder_name,
            pipeline.ffmpeg_path,
            target.fps,
            self.settings.video_bitrate_kbps
        );

        let result = run_ffmpeg(
            &pipeline.ffmpeg_path,
            &self.settings,
            target,
            pipeline.encoder,
            pipeline.codec,
            hub,
        )
        .await;

        if ended_before_the_first_frame(&result) {
            warn!(
                "ffmpeg stopped before the first frame with {encoder_name}, the encoder is checked again before the next start"
            );
            if let Some(picker) = self.picker_for(pipeline.codec) {
                picker.forget_resolved();
            }
        }

        result
    }
}
