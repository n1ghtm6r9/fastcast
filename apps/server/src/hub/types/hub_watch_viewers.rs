use crate::hub::Hub;
use tokio::sync::watch;

impl Hub {
    pub fn watch_viewers(&self) -> watch::Receiver<usize> {
        self.viewers.subscribe()
    }
}
