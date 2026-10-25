use crate::display::{DisplayMode, VirtualDisplay};
use tokio::task::spawn_blocking;
use tracing::warn;

pub async fn open_virtual_display(requested: DisplayMode) -> Option<VirtualDisplay> {
    match spawn_blocking(move || VirtualDisplay::open(requested)).await {
        Ok(Ok(display)) => Some(display),
        Ok(Err(error)) => {
            warn!("no virtual display at {requested}, capturing the monitor instead: {error}");

            None
        }
        Err(error) => {
            warn!("opening the virtual display stopped: {error}");

            None
        }
    }
}
