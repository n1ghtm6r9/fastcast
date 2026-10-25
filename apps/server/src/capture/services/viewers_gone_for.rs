use std::time::Duration;
use tokio::sync::watch;
use tokio::time::timeout;

pub async fn viewers_gone_for(viewers: &mut watch::Receiver<usize>, grace: Duration) {
    loop {
        let counted = viewers.wait_for(|count| *count == 0).await.is_ok();

        if !counted {
            return;
        }

        let returned_in_time = timeout(grace, viewers.wait_for(|count| *count > 0))
            .await
            .is_ok();

        if !returned_in_time {
            return;
        }
    }
}
