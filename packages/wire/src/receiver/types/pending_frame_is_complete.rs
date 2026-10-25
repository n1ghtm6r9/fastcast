use crate::receiver::PendingFrame;

impl PendingFrame {
    pub fn is_complete(&self) -> bool {
        self.completed_at.is_some()
    }
}
