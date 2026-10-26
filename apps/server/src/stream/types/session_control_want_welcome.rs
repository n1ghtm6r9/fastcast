use crate::stream::SessionControl;
use std::sync::atomic::Ordering;

impl SessionControl {
    pub fn want_welcome(&self) {
        self.welcome_wanted.store(true, Ordering::Release);
        self.wake.notify_one();
    }
}
