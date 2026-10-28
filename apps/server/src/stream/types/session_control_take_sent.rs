use crate::stream::SessionControl;
use std::mem;
use std::time::Duration;

impl SessionControl {
    pub fn take_sent(&self) -> Vec<Duration> {
        mem::take(&mut *self.lock_sent())
    }
}
