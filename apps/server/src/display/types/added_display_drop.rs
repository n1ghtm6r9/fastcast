use crate::display::{
    AddedDisplay, EMPTY_IOCTL_INPUT, KEEP_ALIVE_IOCTL, REMOVE_DISPLAY_IOCTL, remove_input_of,
};
use tracing::warn;

impl Drop for AddedDisplay {
    fn drop(&mut self) {
        let removed = self
            .driver
            .request(REMOVE_DISPLAY_IOCTL, remove_input_of(self.index));
        let refreshed = self.driver.request(KEEP_ALIVE_IOCTL, EMPTY_IOCTL_INPUT);

        if let Err(error) = removed.and(refreshed) {
            warn!("could not remove virtual display {}: {error}", self.index);
        }
    }
}
