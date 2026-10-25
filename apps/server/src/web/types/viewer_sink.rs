use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct ViewerSink {
    pub(crate) sink: Mutex<SplitSink<WebSocket, Message>>,
    pub(crate) write_timeout: Duration,
}

impl ViewerSink {
    pub fn new(sink: SplitSink<WebSocket, Message>, write_timeout: Duration) -> Self {
        Self {
            sink: Mutex::new(sink),
            write_timeout,
        }
    }
}
