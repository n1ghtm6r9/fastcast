use crate::stream::SessionControl;

impl SessionControl {
    pub fn forget_sent(&self) {
        self.lock_sent().clear();
    }
}
