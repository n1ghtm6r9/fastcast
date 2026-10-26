use crate::server::ServerStatus;
use crate::server::test::STATUS_LIMIT;
use tokio::sync::watch;
use tokio::time::timeout;

pub async fn status_where(
    status: &mut watch::Receiver<ServerStatus>,
    condition: impl FnMut(&ServerStatus) -> bool,
) -> ServerStatus {
    timeout(STATUS_LIMIT, status.wait_for(condition))
        .await
        .expect("the status comes in time")
        .expect("the status is still sent")
        .clone()
}
