use crate::capture::{NativeCapture, NativeError};
use std::time::Instant;

impl NativeCapture<'_> {
    pub(crate) fn repeat(&mut self, at: Instant) -> Result<(), NativeError> {
        if self.picture.exists() {
            self.encode_and_publish(at)?;
        }

        Ok(())
    }
}
