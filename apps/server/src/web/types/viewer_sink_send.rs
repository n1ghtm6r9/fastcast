use crate::web::ViewerSink;
use axum::extract::ws::Message;
use futures_util::SinkExt;
use std::io::{Error, ErrorKind};
use tokio::time::timeout;

impl ViewerSink {
    pub async fn send(&self, message: Message) -> Result<(), axum::Error> {
        let write = async {
            let mut sink = self.sink.lock().await;

            sink.send(message).await
        };

        match timeout(self.write_timeout, write).await {
            Ok(result) => result,
            Err(_) => Err(axum::Error::new(Error::new(
                ErrorKind::TimedOut,
                "the viewer did not take a message in time",
            ))),
        }
    }
}
