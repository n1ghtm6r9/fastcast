use crate::server::{ServerControl, ServerStatus};
use tokio::sync::watch;

impl ServerControl {
    pub fn watch_status(&self) -> watch::Receiver<ServerStatus> {
        self.status.clone()
    }
}
