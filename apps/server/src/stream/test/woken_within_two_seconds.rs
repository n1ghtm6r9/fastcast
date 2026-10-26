use crate::stream::SessionControl;
use std::time::Duration;
use tokio::time::timeout;

pub async fn woken_within_two_seconds(control: &SessionControl) -> bool {
    timeout(Duration::from_secs(2), control.woken())
        .await
        .is_ok()
}
