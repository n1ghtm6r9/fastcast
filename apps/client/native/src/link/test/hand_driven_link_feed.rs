use crate::link::take_datagram;
use crate::link::test::HandDrivenLink;
use std::time::Duration;

impl HandDrivenLink {
    pub(crate) fn feed(&mut self, datagrams: &[Vec<u8>], at: Duration) {
        for datagram in datagrams {
            take_datagram(
                datagram,
                self.target.server,
                &self.target,
                &self.shared,
                &mut self.state,
                self.start + at,
            );
        }
    }
}
