use crate::capture::{FailedReopen, NativeError, ReopenFailures};
use desktop::DesktopError;
use std::time::Instant;
use tracing::warn;

pub fn note_failed_reopen(
    failures: &mut ReopenFailures,
    now: Instant,
    error: DesktopError,
) -> Result<(), NativeError> {
    match failures.failed(now) {
        FailedReopen::Quiet => Ok(()),
        FailedReopen::Log => {
            warn!("the desktop duplication cannot be opened again yet: {error}");

            Ok(())
        }
        FailedReopen::GiveUp => Err(NativeError::ReopenGaveUp(error)),
    }
}
