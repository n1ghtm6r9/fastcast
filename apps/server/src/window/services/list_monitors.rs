use crate::window::Monitor;
#[cfg(windows)]
use crate::window::monitors_of;
#[cfg(windows)]
use tracing::warn;

#[cfg(windows)]
pub fn list_monitors() -> Vec<Monitor> {
    match desktop::list_outputs() {
        Ok(outputs) => monitors_of(&outputs),
        Err(problem) => {
            warn!("the monitors cannot be listed: {problem}");
            Vec::new()
        }
    }
}

#[cfg(not(windows))]
pub fn list_monitors() -> Vec<Monitor> {
    Vec::new()
}
