use crate::hub::ViewerRequest;
use std::future::pending;
use tokio::sync::watch;

pub async fn request_changed(requests: &mut watch::Receiver<ViewerRequest>) {
    if requests.changed().await.is_err() {
        pending::<()>().await;
    }
}
