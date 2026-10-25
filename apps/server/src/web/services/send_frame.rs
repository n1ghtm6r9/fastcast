use crate::hub::Frame;
use axum::extract::ws::{Message, WebSocket};
use futures_util::SinkExt;
use futures_util::stream::SplitSink;

pub async fn send_frame(
    sink: &mut SplitSink<WebSocket, Message>,
    frame: &Frame,
) -> Result<(), axum::Error> {
    sink.send(Message::Binary(frame.wire.clone())).await
}
