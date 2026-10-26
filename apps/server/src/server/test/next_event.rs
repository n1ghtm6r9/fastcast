use crate::server::test::{FakeEvent, STATUS_LIMIT};
use tokio::sync::mpsc;
use tokio::time::timeout;

pub async fn next_event(events: &mut mpsc::UnboundedReceiver<FakeEvent>) -> FakeEvent {
    timeout(STATUS_LIMIT, events.recv())
        .await
        .expect("the event comes in time")
        .expect("the fake server is alive")
}
