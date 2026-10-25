use crate::hub::StreamConfig;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;

pub async fn send_config(
    sink: &mut SplitSink<WebSocket, Message>,
    config: &StreamConfig,
) -> Result<(), axum::Error> {
    sink.send(Message::Text(config.json.clone().into())).await
}
