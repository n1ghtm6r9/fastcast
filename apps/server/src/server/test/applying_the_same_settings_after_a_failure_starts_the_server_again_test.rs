use crate::server::ServerState;
use crate::server::test::{
    BUSY_NAME, next_event, settings_named, start_fake_control, started, status_where,
};

#[tokio::test]
async fn applying_the_same_settings_after_a_failure_starts_the_server_again() {
    let (control, mut events) = start_fake_control(BUSY_NAME);

    assert_eq!(next_event(&mut events).await, started(BUSY_NAME));
    status_where(&mut control.watch_status(), |status| {
        matches!(status.state, ServerState::Failed { .. })
    })
    .await;

    control.apply(settings_named(BUSY_NAME));

    assert_eq!(next_event(&mut events).await, started(BUSY_NAME));

    control.stop().await;
}
