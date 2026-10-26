use crate::capture::{CaptureError, CaptureReport};
use crate::hub::Subscription;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tokio::time::{Instant, timeout_at};

pub async fn frames_within_while_running<R>(
    mut run: Pin<&mut R>,
    subscription: &mut Subscription,
    window: Duration,
) -> usize
where
    R: Future<Output = Result<CaptureReport, CaptureError>>,
{
    let deadline = Instant::now() + window;
    let mut frames = 0;

    loop {
        tokio::select! {
            result = run.as_mut() => panic!("the native run ended inside the window: {result:?}"),
            received = timeout_at(deadline, subscription.frames.recv()) => match received {
                Ok(Ok(_)) => frames += 1,
                Ok(Err(error)) => panic!("the hub stopped sending frames: {error}"),
                Err(_) => return frames,
            },
        }
    }
}
