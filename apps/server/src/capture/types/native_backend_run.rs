use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, CaptureTarget, NativeBackend, NativeError,
    NativePipeline, Screen, StopOnDrop, bit_depth_for, ensure_native_encoder, open_screen,
    run_native_capture, spawn_capture_worker, stream_bitrate_kbps,
};
use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::oneshot;
use tracing::info;

impl CaptureBackend for NativeBackend {
    type Pipeline = NativePipeline;

    async fn prepare(&self, request: &ViewerRequest) -> Result<NativePipeline, CaptureError> {
        ensure_native_encoder(self.settings.video_encoder)?;

        let codec = if request.codecs.contains(&VideoCodec::Hevc) {
            VideoCodec::Hevc
        } else {
            VideoCodec::Avc
        };

        Ok(NativePipeline {
            codec,
            bit_depth: bit_depth_for(self.settings.ten_bit_video, codec, request),
        })
    }

    async fn open_screen(&self, request: &ViewerRequest) -> Screen {
        open_screen(&self.settings, request).await
    }

    async fn run(
        &self,
        hub: &Arc<Hub>,
        pipeline: &NativePipeline,
        target: &CaptureTarget,
    ) -> Result<CaptureReport, CaptureError> {
        let bitrate_kbps = stream_bitrate_kbps(self.settings.video_bitrate_kbps, target.fps);

        info!(
            "starting native capture of output {} with NVENC {:?} {}-bit at {} fps, {} kbps",
            target.output, pipeline.codec, pipeline.bit_depth, target.fps, bitrate_kbps
        );

        let stop = StopOnDrop(Arc::new(AtomicBool::new(false)));
        let previous = self.take_worker();
        let (sender, receiver) = oneshot::channel();

        let spawned = spawn_capture_worker(previous, {
            let hub = hub.clone();
            let pipeline = *pipeline;
            let target = target.clone();
            let stopped = stop.0.clone();

            move || {
                let result = run_native_capture(&hub, pipeline, &target, bitrate_kbps, &stopped);

                let _ = sender.send(result);
            }
        });

        match spawned {
            Ok(worker) => self.keep_worker(worker),
            Err(source) => {
                return Err(CaptureError::Native {
                    frames: 0,
                    cause: NativeError::Spawn(source),
                });
            }
        }

        receiver.await.unwrap_or(Err(CaptureError::Native {
            frames: 0,
            cause: NativeError::WorkerGone,
        }))
    }
}
