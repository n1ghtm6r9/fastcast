use crate::web::{VIEWER_REPORT_LIMIT, clipped};
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tracing::info;

pub async fn watch_for_close(mut stream: SplitStream<WebSocket>) {
    while let Some(message) = stream.next().await {
        match message {
            Ok(Message::Close(_)) | Err(_) => return,
            Ok(Message::Text(report)) => {
                info!(
                    "viewer report: {}",
                    clipped(report.as_str(), VIEWER_REPORT_LIMIT)
                );
            }
            Ok(_) => {}
        }
    }
}
