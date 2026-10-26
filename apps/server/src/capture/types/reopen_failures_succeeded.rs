use crate::capture::ReopenFailures;

impl ReopenFailures {
    pub fn succeeded(&mut self) {
        *self = Self::default();
    }
}
