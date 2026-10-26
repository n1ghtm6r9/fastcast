use crate::hub::Hub;
use tokio::sync::watch;

impl Hub {
    pub fn watch_reports(&self) -> watch::Receiver<Option<String>> {
        self.reports.subscribe()
    }
}
