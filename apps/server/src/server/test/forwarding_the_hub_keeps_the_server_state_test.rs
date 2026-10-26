use crate::hub::Hub;
use crate::server::test::{FAKE_ADDRESS, status_where};
use crate::server::{ServerState, ServerStatus, forward_hub_status};
use std::sync::Arc;
use tokio::sync::watch;

#[tokio::test]
async fn forwarding_the_hub_keeps_the_server_state() {
    let hub = Arc::new(Hub::new());
    let running = ServerState::Running {
        address: FAKE_ADDRESS,
    };
    let (status, mut watched) = watch::channel(ServerStatus {
        state: running.clone(),
        ..ServerStatus::default()
    });
    let forwarding = tokio::spawn(forward_hub_status(hub.clone(), status));

    let _viewer = hub.join();

    assert_eq!(
        status_where(&mut watched, |status| status.viewers > 0)
            .await
            .state,
        running
    );

    forwarding.abort();
}
