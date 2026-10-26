use crate::hub::Hub;
use crate::server::test::status_where;
use crate::server::{ServerStatus, forward_hub_status};
use std::sync::Arc;
use tokio::sync::watch;

#[tokio::test]
async fn the_status_shows_the_last_tv_report() {
    let hub = Arc::new(Hub::new());
    let (status, mut watched) = watch::channel(ServerStatus::default());
    let forwarding = tokio::spawn(forward_hub_status(hub.clone(), status));

    hub.report("{\"decoded\":300,\"dropped\":0}".to_owned());

    assert_eq!(
        status_where(&mut watched, |status| status.report.is_some())
            .await
            .report
            .as_deref(),
        Some("{\"decoded\":300,\"dropped\":0}")
    );

    forwarding.abort();
}
