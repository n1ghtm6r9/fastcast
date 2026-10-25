use crate::capture::{RETRY_DELAY_AFTER_FAILURE, request_changed};
use crate::hub::ViewerRequest;
use tokio::sync::watch;
use tokio::time::sleep;

pub async fn wait_before_retry(requests: &mut watch::Receiver<ViewerRequest>) {
    tokio::select! {
        () = sleep(RETRY_DELAY_AFTER_FAILURE) => {}
        () = request_changed(requests) => {}
    }
}
