use crate::web::{VIEWER_CLOSE_TIMEOUT, ViewerSink};
use futures_util::SinkExt;
use tokio::time::timeout;

impl ViewerSink {
    pub async fn close(&self) {
        let closing = async {
            let mut sink = self.sink.lock().await;

            sink.close().await
        };

        let _ = timeout(VIEWER_CLOSE_TIMEOUT, closing).await;
    }
}
