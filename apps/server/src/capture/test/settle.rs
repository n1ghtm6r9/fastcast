use std::time::Duration;
use tokio::time::sleep;

pub async fn settle() {
    sleep(Duration::from_millis(1)).await;
}
