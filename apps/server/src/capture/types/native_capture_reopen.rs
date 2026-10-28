use crate::capture::{NativeCapture, NativeError, ReopenStep, note_failed_reopen, reopen_step_of};
use desktop::Duplication;
use std::thread::sleep;
use std::time::Instant;
use tracing::warn;

impl NativeCapture<'_> {
    pub(crate) fn reopen(&mut self) -> Result<ReopenStep, NativeError> {
        let reopened = Duplication::new(&self.device, &self.output);
        let step = reopen_step_of(
            self.size,
            reopened
                .as_ref()
                .map(|duplication| (duplication.width(), duplication.height())),
        );

        match reopened {
            Ok(duplication) => {
                self.reopen_failures.succeeded();

                if step == ReopenStep::Resume {
                    self.duplication = Some(duplication);
                }
            }
            Err(error) if step == ReopenStep::Restart => warn!(
                "the desktop duplication cannot be opened again on this graphics device: {error}, \
                 starting the capture again on a new one"
            ),
            Err(error) => {
                note_failed_reopen(&mut self.reopen_failures, Instant::now(), error)?;
                sleep(self.clock.until_repeat(Instant::now()));
                self.repeat(Instant::now())?;
            }
        }

        Ok(step)
    }
}
