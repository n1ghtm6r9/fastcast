use crate::display::{
    ADD_DISPLAY_IOCTL, AddedDisplay, DisplayError, DriverHandle, EMPTY_IOCTL_INPUT,
    KEEP_ALIVE_IOCTL, KeepAlive,
};
use std::sync::Arc;

impl AddedDisplay {
    pub fn add(driver: Arc<DriverHandle>) -> Result<AddedDisplay, DisplayError> {
        let keep_alive = KeepAlive::start(Arc::clone(&driver))?;
        let reported = driver.request(ADD_DISPLAY_IOCTL, EMPTY_IOCTL_INPUT)?;
        let index =
            u16::try_from(reported).map_err(|_| DisplayError::DriverRefusedToAdd { reported })?;
        let added = AddedDisplay {
            _keep_alive: keep_alive,
            driver,
            index,
        };

        added.driver.request(KEEP_ALIVE_IOCTL, EMPTY_IOCTL_INPUT)?;

        Ok(added)
    }
}
