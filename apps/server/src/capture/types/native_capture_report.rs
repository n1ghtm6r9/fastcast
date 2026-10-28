use crate::capture::NativeCapture;
use std::time::Instant;
use tracing::info;

impl NativeCapture<'_> {
    pub(crate) fn report(&mut self) {
        if let Some(line) = self.stats.line(Instant::now()) {
            info!("{line}");
        }
    }
}
