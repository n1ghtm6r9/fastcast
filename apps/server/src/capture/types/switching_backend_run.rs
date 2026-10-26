use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, CaptureTarget, Screen, SwitchedPipeline,
    SwitchingBackend,
};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use tracing::warn;

impl<N: CaptureBackend, F: CaptureBackend> CaptureBackend for SwitchingBackend<N, F> {
    type Pipeline = SwitchedPipeline<N::Pipeline, F::Pipeline>;

    async fn prepare(&self, request: &ViewerRequest) -> Result<Self::Pipeline, CaptureError> {
        if self.has_failed_for(request) {
            return Ok(SwitchedPipeline::Ffmpeg(
                self.ffmpeg.prepare(request).await?,
            ));
        }

        match self.native.prepare(request).await {
            Ok(pipeline) => Ok(SwitchedPipeline::Native {
                pipeline,
                request: request.clone(),
            }),
            Err(failure) => {
                warn!("native capture is not available ({failure}), using ffmpeg");

                Ok(SwitchedPipeline::Ffmpeg(
                    self.ffmpeg.prepare(request).await?,
                ))
            }
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
                    Err(CaptureError::Native { frames: 0, cause }) => {
                        self.note_native_failure(request, &cause);

                        let pipeline = self.ffmpeg.prepare(request).await?;

                        self.ffmpeg.run(hub, &pipeline, target).await
                    }
                    other => other,
                }
            }
        }
    }
}
