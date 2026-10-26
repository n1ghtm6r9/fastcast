use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, CaptureTarget, Screen, SwitchedPipeline,
    SwitchingBackend,
};
use crate::hub::{Hub, ViewerRequest};
use crate::settings::BackendChoice;
use std::sync::Arc;
use tracing::warn;

impl<N: CaptureBackend, F: CaptureBackend> CaptureBackend for SwitchingBackend<N, F> {
    type Pipeline = SwitchedPipeline<N::Pipeline, F::Pipeline>;

    async fn prepare(&self, request: &ViewerRequest) -> Result<Self::Pipeline, CaptureError> {
        let native_is_wanted = match self.choice {
            BackendChoice::Ffmpeg => false,
            BackendChoice::Native => true,
            BackendChoice::Auto => !self.has_failed_for(request),
        };

        if !native_is_wanted {
            return Ok(SwitchedPipeline::Ffmpeg(
                self.ffmpeg.prepare(request).await?,
            ));
        }

        match self.native.prepare(request).await {
            Ok(pipeline) => Ok(SwitchedPipeline::Native {
                pipeline,
                request: request.clone(),
            }),
            Err(failure) if self.choice == BackendChoice::Auto => {
                warn!("native capture is not available ({failure}), using ffmpeg");

                Ok(SwitchedPipeline::Ffmpeg(
                    self.ffmpeg.prepare(request).await?,
                ))
            }
            Err(failure) => Err(failure),
        }
    }

    async fn open_screen(&self, request: &ViewerRequest) -> Screen {
        self.ffmpeg.open_screen(request).await
    }

    async fn run(
        &self,
        hub: &Arc<Hub>,
        pipeline: &Self::Pipeline,
        target: &CaptureTarget,
    ) -> Result<CaptureReport, CaptureError> {
        match pipeline {
            SwitchedPipeline::Ffmpeg(pipeline) => self.ffmpeg.run(hub, pipeline, target).await,
            SwitchedPipeline::Native { pipeline, request } => {
                match self.native.run(hub, pipeline, target).await {
                    Err(CaptureError::Native { frames: 0, cause })
                        if self.choice == BackendChoice::Auto =>
                    {
                        self.remember_failure_for(request);

                        warn!(
                            "native capture failed before its first frame ({cause}), using ffmpeg for this picture"
                        );

                        let pipeline = self.ffmpeg.prepare(request).await?;

                        self.ffmpeg.run(hub, &pipeline, target).await
                    }
                    other => other,
                }
            }
        }
    }
}
