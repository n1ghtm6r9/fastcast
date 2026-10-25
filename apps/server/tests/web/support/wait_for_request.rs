use app::hub::{Hub, ViewerRequest};
use std::time::Duration;
use tokio::time::timeout;

pub async fn wait_for_request(hub: &Hub, request: &ViewerRequest) {
    let mut requests = hub.watch_requests();

    timeout(
        Duration::from_secs(5),
        requests.wait_for(|current| current == request),
    )
    .await
    .expect("the request arrives in time")
    .expect("the hub is alive");
}
