use super::{FakeBackend, FakePreparation, FakeRun, KillCounter, capture_target};
use crate::capture::{
    CaptureBackend, CaptureError, CaptureReport, CaptureTarget, NativeError, Screen,
};
use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::time::sleep;

impl CaptureBackend for FakeBackend {
    type Pipeline = ();

    async fn prepare(&self, _request: &ViewerRequest) -> Result<(), CaptureError> {
        self.prepared.fetch_add(1, Ordering::SeqCst);

        let preparation = self
            .preparations
            .lock()
            .expect("the preparations lock is not poisoned")
            .pop_front()
            .unwrap_or(FakePreparation::Succeeds);

        match preparation {
            FakePreparation::Succeeds => Ok(()),
            FakePreparation::Takes { delay } => {
                sleep(delay).await;
                Ok(())
            }
            FakePreparation::Fails => Err(CaptureError::FfmpegNotFound {
                path: "fastcast-fake-ffmpeg".to_owned(),
            }),
        }
    }

    async fn open_screen(&self, request: &ViewerRequest) -> Screen {
        self.opened
            .lock()
            .expect("the opened lock is not poisoned")
            .push(request.clone());

        Screen {
            request: request.clone(),
            target: capture_target(),
            display: None,
        }
    }

    async fn run(
        &self,
        hub: &Arc<Hub>,
        _pipeline: &(),
        _target: &CaptureTarget,
    ) -> Result<CaptureReport, CaptureError> {
        self.started.fetch_add(1, Ordering::SeqCst);

        let run = self
            .script
            .lock()
            .expect("the script lock is not poisoned")
            .pop_front()
            .unwrap_or(FakeRun::Forever);

        hub.set_config(config_of(&nvenc_2560x1440_avcc(), 60));
        hub.publish(true, 0, &[1]);

        let mut kill_counter = KillCounter::armed(self.killed.clone());

        let result = match run {
            FakeRun::Forever => std::future::pending().await,
            FakeRun::EndsAfter { delay, frames } => {
                sleep(delay).await;
                Ok(CaptureReport { frames })
            }
            FakeRun::Fails { delay } => {
                sleep(delay).await;
                Err(CaptureError::MissingPipe)
            }
            FakeRun::FailsNatively { frames } => Err(CaptureError::Native {
                frames,
                cause: NativeError::NotNvidia {
                    adapter: "a fake adapter".to_owned(),
                },
            }),
            FakeRun::FailsNativelyTemporarily { frames } => Err(CaptureError::Native {
                frames,
                cause: NativeError::WorkerGone,
            }),
        };

        kill_counter.disarm();

        result
    }
}
