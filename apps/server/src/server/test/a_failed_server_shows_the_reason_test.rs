use crate::server::ServerState;
use crate::server::test::{
    BUSY_NAME, busy_port_error, next_event, start_fake_control, started, status_where,
};

#[tokio::test]
async fn a_failed_server_shows_the_reason() {
    let (control, mut events) = start_fake_control(BUSY_NAME);

    assert_eq!(next_event(&mut events).await, started(BUSY_NAME));

    let failed = status_where(&mut control.watch_status(), |status| {
        status.state != ServerState::Starting
    })
    .await;

    assert_eq!(
        failed.state,
        ServerState::Failed {
            reason: busy_port_error().to_string()
        }
    );

    control.stop().await;
}
