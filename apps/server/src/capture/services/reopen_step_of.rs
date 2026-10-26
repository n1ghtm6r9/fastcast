use crate::capture::ReopenStep;

pub fn reopen_step_of(expected: (u32, u32), reopened: Option<(u32, u32)>) -> ReopenStep {
    match reopened {
        None => ReopenStep::Retry,
        Some(size) if size == expected => ReopenStep::Resume,
        Some(_) => ReopenStep::Restart,
    }
}
