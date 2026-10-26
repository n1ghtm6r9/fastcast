use crate::link::test::{HandDrivenLink, TEST_SESSION};
use std::sync::atomic::Ordering;

impl HandDrivenLink {
    pub(crate) fn welcome(&mut self) {
        self.state.session = Some(TEST_SESSION);
        self.shared.session.store(TEST_SESSION, Ordering::Release);
    }
}
