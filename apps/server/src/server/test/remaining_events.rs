use crate::server::test::{FakeEvent, STATUS_LIMIT};
use tokio::sync::mpsc;
use tokio::time::timeout;

pub async fn remaining_events(mut events: mpsc::UnboundedReceiver<FakeEvent>) -> Vec<FakeEvent> {
    let mut remaining = Vec::new();

    while let Some(event) = timeout(STATUS_LIMIT, events.recv())
        .await
        .expect("the events end in time")
    {
        remaining.push(event);
    }

    remaining
}
