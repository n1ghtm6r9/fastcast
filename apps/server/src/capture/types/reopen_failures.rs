use std::time::Instant;

#[derive(Default)]
pub struct ReopenFailures {
    pub(crate) since: Option<Instant>,
    pub(crate) logged: Option<Instant>,
}
