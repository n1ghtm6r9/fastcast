use crate::support::{ReceivedFrame, Viewer, next_message, received_frame_of};
use tokio_tungstenite::tungstenite::Message;

pub async fn next_frame(viewer: &mut Viewer) -> ReceivedFrame {
    match next_message(viewer).await {
        Message::Binary(wire) => received_frame_of(&wire),
        other => panic!("expected a binary frame, got {other:?}"),
    }
}
