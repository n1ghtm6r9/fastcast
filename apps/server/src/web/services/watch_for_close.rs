use crate::web::{VIEWER_REPORT_LIMIT, clipped};
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use std::time::Duration;
use tokio::time::timeout;
use tracing::{debug, info};

pub async fn watch_for_close(mut stream: SplitStream<WebSocket>, silence_timeout: Duration) {
    loop {
        match timeout(silence_timeout, stream.next()).await {
            Ok(Some(Ok(Message::Close(_)))) | Ok(Some(Err(_))) | Ok(None) => return,
            Ok(Some(Ok(Message::Text(report)))) => {
                info!(
                    "viewer report: {}",
                    clipped(report.as_str(), VIEWER_REPORT_LIMIT)
                );
            }
            Ok(Some(Ok(_))) => {}
            Err(_) => {
                debug!("a viewer sent nothing for {silence_timeout:?}");

                return;
            }
        }
    }
}
