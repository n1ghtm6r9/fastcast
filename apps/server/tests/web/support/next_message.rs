use crate::support::Viewer;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

pub async fn next_message(viewer: &mut Viewer) -> Message {
    timeout(Duration::from_secs(5), viewer.next())
        .await
        .expect("a message arrives in time")
        .expect("the connection stays open")
        .expect("the message is valid")
}
