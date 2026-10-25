use crate::web::ViewerSink;
use axum::extract::ws::Message;
use bytes::Bytes;
use std::time::Duration;
use tokio::time::{Instant, MissedTickBehavior, interval_at};

pub async fn ping_viewer(sink: &ViewerSink, ping_interval: Duration) -> axum::Error {
    let mut ticks = interval_at(Instant::now() + ping_interval, ping_interval);
    ticks.set_missed_tick_behavior(MissedTickBehavior::Delay);

    loop {
        ticks.tick().await;

        if let Err(error) = sink.send(Message::Ping(Bytes::new())).await {
            return error;
        }
    }
}
