use crate::stream::ReceiverState;
use std::net::SocketAddr;

impl ReceiverState {
    pub(crate) fn bye(&mut self, peer: SocketAddr, session: u32) {
        if self
            .sessions
            .get(&peer)
            .is_some_and(|entry| entry.id == session)
        {
            self.end_session(peer, "left");
        }
    }
}
