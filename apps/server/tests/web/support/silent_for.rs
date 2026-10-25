use crate::support::Viewer;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;

pub async fn silent_for(viewer: &mut Viewer, wait: Duration) -> bool {
    timeout(wait, viewer.next()).await.is_err()
}
