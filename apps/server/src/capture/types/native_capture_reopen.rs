use crate::capture::{NativeCapture, NativeError, ReopenStep, note_failed_reopen, reopen_step_of};
use desktop::Duplication;
use std::thread::sleep;
use std::time::Instant;

impl NativeCapture<'_> {
    pub(crate) fn reopen(&mut self) -> Result<ReopenStep, NativeError> {
        let reopened = match Duplication::new(&self.device, &self.output) {
            Ok(duplication) => {
                self.reopen_failures.succeeded();

                Some(duplication)
            }
            Err(error) => {
                note_failed_reopen(&mut self.reopen_failures, Instant::now(), error)?;

                None
            }
        };
        let size = reopened
            .as_ref()
            .map(|duplication| (duplication.width(), duplication.height()));
        let step = reopen_step_of(self.size, size);

        match step {
            ReopenStep::Resume => self.duplication = reopened,
            ReopenStep::Retry => {
                sleep(self.clock.until_repeat(Instant::now()));
                self.repeat(Instant::now())?;
            }
            ReopenStep::Restart => {}
        }

        Ok(step)
    }
}
