use crate::server::ServerControl;
use tracing::warn;

impl ServerControl {
    pub async fn stop(self) {
        let ServerControl {
            settings, driver, ..
        } = self;

        drop(settings);

        if let Err(error) = driver.await {
            warn!("the server control did not stop cleanly: {error}");
        }
    }
}
