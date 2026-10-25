use crate::hub::StreamConfig;
use crate::web::ViewerSink;
use axum::extract::ws::Message;

pub async fn send_config(sink: &ViewerSink, config: &StreamConfig) -> Result<(), axum::Error> {
    sink.send(Message::Text(config.json.clone().into())).await
}
