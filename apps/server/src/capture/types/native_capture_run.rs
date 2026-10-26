use crate::capture::{NativeCapture, NativeError, ReopenStep};

impl NativeCapture<'_> {
    pub fn run(&mut self) -> Result<(), NativeError> {
        while !self.stopped() {
            match self.duplication.take() {
                Some(duplication) => self.capture_from(duplication)?,
                None => {
                    if self.reopen()? == ReopenStep::Restart {
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
