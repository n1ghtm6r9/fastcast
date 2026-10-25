use crate::display::VirtualDisplay;
use tracing::info;

impl Drop for VirtualDisplay {
    fn drop(&mut self) {
        info!(
            "removing virtual display {} and turning the monitors back on",
            self.device_name
        );
    }
}
