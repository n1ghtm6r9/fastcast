use crate::server::ServerState;
use crate::server::test::{next_event, start_fake_control, started, stopped};

#[tokio::test]
async fn stopping_the_control_stops_the_server_before_it_returns() {
    let (control, mut events) = start_fake_control("first");
    let status = control.watch_status();

    assert_eq!(next_event(&mut events).await, started("first"));

    control.stop().await;

    assert_eq!(events.try_recv(), Ok(stopped("first")));
    assert_eq!(status.borrow().state, ServerState::Stopped);
}
