use crate::display::VirtualDisplay;
use tracing::info;

impl Drop for VirtualDisplay {
    fn drop(&mut self) {
        info!(
            "removing virtual display {} (DXGI output {})",
            self.device_name, self.output_index
        );
    }
}
