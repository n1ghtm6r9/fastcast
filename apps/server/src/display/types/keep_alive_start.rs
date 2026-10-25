use crate::display::{DisplayError, DriverHandle, KeepAlive, run_keep_alive, system_error};
use std::sync::{Arc, mpsc};
use std::thread;

impl KeepAlive {
    pub fn start(driver: Arc<DriverHandle>) -> Result<KeepAlive, DisplayError> {
        let (stop, stopped) = mpsc::channel();
        let thread = thread::Builder::new()
            .name("virtual-display-keep-alive".to_owned())
            .spawn(move || run_keep_alive(&driver, &stopped))
            .map_err(system_error("starting the keep-alive thread"))?;

        Ok(KeepAlive {
            stop: Some(stop),
            thread: Some(thread),
        })
    }
}
