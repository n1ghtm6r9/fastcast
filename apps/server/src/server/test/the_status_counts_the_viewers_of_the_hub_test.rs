use crate::hub::Hub;
use crate::server::test::status_where;
use crate::server::{ServerStatus, forward_hub_status};
use std::sync::Arc;
use tokio::sync::watch;

#[tokio::test]
async fn the_status_counts_the_viewers_of_the_hub() {
    let hub = Arc::new(Hub::new());
    let (status, mut watched) = watch::channel(ServerStatus::default());
    let forwarding = tokio::spawn(forward_hub_status(hub.clone(), status));

    let _viewer = hub.join();

    assert_eq!(
        status_where(&mut watched, |status| status.viewers > 0)
            .await
            .viewers,
        1
    );

    forwarding.abort();
}
