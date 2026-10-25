use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use futures_util::stream::SplitStream;

pub async fn watch_for_close(mut stream: SplitStream<WebSocket>) {
    while let Some(message) = stream.next().await {
        match message {
            Ok(Message::Close(_)) | Err(_) => return,
            Ok(_) => {}
        }
    }
}
