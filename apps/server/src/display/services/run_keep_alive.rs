use crate::display::{DriverHandle, EMPTY_IOCTL_INPUT, KEEP_ALIVE_INTERVAL, KEEP_ALIVE_IOCTL};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use tracing::warn;

pub fn run_keep_alive(driver: &DriverHandle, stopped: &Receiver<()>) {
    let mut reported = false;

    loop {
        if let Err(error) = driver.request(KEEP_ALIVE_IOCTL, EMPTY_IOCTL_INPUT)
            && !reported
        {
            warn!("the virtual display keep-alive failed: {error}");
            reported = true;
        }

        if !matches!(
            stopped.recv_timeout(KEEP_ALIVE_INTERVAL),
            Err(RecvTimeoutError::Timeout)
        ) {
            return;
        }
    }
}
