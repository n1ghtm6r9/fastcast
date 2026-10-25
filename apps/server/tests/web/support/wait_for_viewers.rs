use app::hub::Hub;
use std::time::Duration;
use tokio::time::timeout;

pub async fn wait_for_viewers(hub: &Hub, count: usize) {
    let mut viewers = hub.watch_viewers();

    timeout(
        Duration::from_secs(5),
        viewers.wait_for(|current| *current == count),
    )
    .await
    .expect("the viewer count settles in time")
    .expect("the hub is alive");
}
