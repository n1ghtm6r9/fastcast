use crate::capture::{NativeCapture, NativeError};
use desktop::Duplication;

impl NativeCapture<'_> {
    pub(crate) fn convert_waiting(&mut self, duplication: &Duplication) -> Result<(), NativeError> {
        if self.picture.take_conversion() {
            self.converter
                .convert(&self.device, duplication.frame(), duplication.pointer())
                .map_err(NativeError::Desktop)?;
        }

        Ok(())
    }
}
