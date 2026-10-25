use crate::support::RunningServer;
use app::web::WebError;
use std::time::Duration;
use tokio::time::timeout;

impl RunningServer {
    pub async fn stop(self) -> Result<(), WebError> {
        let _ = self.shutdown.send(());

        timeout(Duration::from_secs(5), self.task)
            .await
            .expect("the server stops in time")
            .expect("the server task does not panic")
    }
}
