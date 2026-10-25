use crate::support::{Viewer, next_message};
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

pub async fn next_config(viewer: &mut Viewer) -> Value {
    match next_message(viewer).await {
        Message::Text(text) => serde_json::from_str(text.as_str()).expect("the config is JSON"),
        other => panic!("expected the config as text, got {other:?}"),
    }
}
