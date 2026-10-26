use crate::server::ServerState;
use crate::server::test::{FAKE_ADDRESS, next_event, start_fake_control, started, status_where};

#[tokio::test]
async fn the_control_starts_the_server_with_its_settings() {
    let (control, mut events) = start_fake_control("first");

    assert_eq!(next_event(&mut events).await, started("first"));

    let running = status_where(&mut control.watch_status(), |status| {
        status.state != ServerState::Starting
    })
    .await;

    assert_eq!(
        running.state,
        ServerState::Running {
            address: FAKE_ADDRESS
        }
    );

    control.stop().await;
}
