use crate::stream::StreamEndpoint;
use std::sync::atomic::Ordering;
use tracing::warn;

impl StreamEndpoint {
    pub(crate) fn shutdown(&mut self) {
        self.stop.store(true, Ordering::Release);

        if let Some(receiver) = self.receiver.take()
            && receiver.join().is_err()
        {
            warn!("the stream receiver thread panicked");
        }
    }
}
