use crate::stream::{MAX_SESSIONS, ReceiverState};
use std::net::SocketAddr;
use std::time::Instant;
use tracing::warn;
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

        if self.sessions.len() >= MAX_SESSIONS && !self.sessions.contains_key(&peer) {
            warn!(%peer, "refused a hello: {MAX_SESSIONS} sessions are live");

            return;
        }

        self.start_session(peer, hello, now);
    }
}
