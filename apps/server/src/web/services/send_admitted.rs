use crate::hub::Frame;
use crate::web::{Admission, ViewerCursor, send_config, send_frame};
use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;

pub async fn send_admitted(
    sink: &mut SplitSink<WebSocket, Message>,
    cursor: &mut ViewerCursor,
    frame: &Frame,
) -> Result<(), axum::Error> {
    match cursor.admit(frame) {
        Admission::Skip => Ok(()),
        Admission::Send => send_frame(sink, frame).await,
        Admission::SendWithConfig(config) => {
            send_config(sink, &config).await?;
            send_frame(sink, frame).await
        }
    }
}
