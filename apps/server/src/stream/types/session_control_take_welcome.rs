use crate::stream::SessionControl;
use std::sync::atomic::Ordering;

impl SessionControl {
    pub fn take_welcome(&self) -> bool {
        self.welcome_wanted.swap(false, Ordering::AcqRel)
    }
}
