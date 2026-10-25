use crate::hub::Frame;
use crate::web::ViewerSink;
use axum::extract::ws::Message;

pub async fn send_frame(sink: &ViewerSink, frame: &Frame) -> Result<(), axum::Error> {
    sink.send(Message::Binary(frame.wire.clone())).await
}
