use crate::link::send_hello;
use crate::link::test::HandDrivenLink;
use std::time::Duration;

impl HandDrivenLink {
    pub(crate) fn hello_at(&mut self, at: Duration) {
        send_hello(
            &self.socket,
            &self.target,
            &self.shared,
            &mut self.state,
            self.start + at,
        );
    }
}
