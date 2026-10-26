use crate::capture::{NativeCapture, NativeError};
use desktop::{DesktopError, Duplication};
use std::time::Instant;

impl NativeCapture<'_> {
    pub(crate) fn capture_from(&mut self, mut duplication: Duplication) -> Result<(), NativeError> {
        let acquired = duplication.acquire(&self.device, self.clock.until_repeat(Instant::now()));
        let arrived = Instant::now();

        match acquired {
            Ok(Some(_)) => {
                self.converter
                    .convert(&self.device, duplication.frame(), duplication.pointer())
                    .map_err(NativeError::Desktop)?;
                self.duplication = Some(duplication);
                self.has_picture = true;

                if self.clock.may_encode(arrived) {
                    self.encode_and_publish(arrived)?;
                }

                Ok(())
            }
            Ok(None) => {
                self.duplication = Some(duplication);

                self.repeat(arrived)
            }
            Err(DesktopError::AccessLost) => Ok(()),
            Err(error) => Err(NativeError::Desktop(error)),
        }
    }
}
