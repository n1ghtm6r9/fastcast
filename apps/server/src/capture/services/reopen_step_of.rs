use crate::capture::ReopenStep;
use desktop::DesktopError;

pub fn reopen_step_of(
    expected: (u32, u32),
    reopened: Result<(u32, u32), &DesktopError>,
) -> ReopenStep {
    match reopened {
        Err(error) if error.is_temporary() => ReopenStep::Retry,
        Err(_) => ReopenStep::Restart,
        Ok(size) if size == expected => ReopenStep::Resume,
        Ok(_) => ReopenStep::Restart,
    }
}
