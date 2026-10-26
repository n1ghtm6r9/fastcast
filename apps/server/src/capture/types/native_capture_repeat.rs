use crate::capture::{NativeCapture, NativeError};
use std::time::Instant;

impl NativeCapture<'_> {
    pub(crate) fn repeat(&mut self, at: Instant) -> Result<(), NativeError> {
        if self.has_picture {
            self.encode_and_publish(at)?;
        }

        Ok(())
    }
}
