use crate::display::DisplayScale;
use crate::hub::Hub;
use tokio::sync::watch;

impl Hub {
    pub fn watch_scale(&self) -> watch::Receiver<Option<DisplayScale>> {
        self.scale.subscribe()
    }
}
