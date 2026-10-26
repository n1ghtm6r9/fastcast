use super::NATIVE_FRAME_WAIT;
use crate::capture::{CaptureError, CaptureReport};
use crate::hub::{Frame, Subscription};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::time::timeout;

pub async fn frame_while_running<R>(run: Pin<&mut R>, subscription: &mut Subscription) -> Arc<Frame>
where
    R: Future<Output = Result<CaptureReport, CaptureError>>,
{
    tokio::select! {
        result = run => panic!("the native run ended before the frame: {result:?}"),
        received = timeout(NATIVE_FRAME_WAIT, subscription.frames.recv()) => received
            .expect("no frame came within five seconds")
            .expect("the hub stopped sending frames"),
    }
}
