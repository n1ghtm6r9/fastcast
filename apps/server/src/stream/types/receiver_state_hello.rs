use crate::stream::ReceiverState;
use std::net::SocketAddr;
use std::time::Instant;
use wire::Hello;

impl ReceiverState {
    pub(crate) fn hello(&mut self, peer: SocketAddr, hello: &Hello, now: Instant) {
        if let Some(entry) = self.sessions.get_mut(&peer)
            && entry.nonce == hello.nonce
        {
            entry.last_seen = now;
            entry.control.want_welcome();

            return;
        }

        self.start_session(peer, hello, now);
    }
}
