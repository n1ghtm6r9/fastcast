use crate::stream::{ReceiverState, scale_step_of};
use std::net::SocketAddr;
use std::time::Instant;

impl ReceiverState {
    pub(crate) fn notice(&mut self, peer: SocketAddr, session: u32, text: &str, now: Instant) {
        let Some(entry) = self
            .sessions
            .get_mut(&peer)
            .filter(|entry| entry.id == session)
        else {
            self.refuse_stranger(peer, session);

            return;
        };

        entry.last_seen = now;

        if let Some(step) = scale_step_of(text) {
            self.hub.ask_scale(step);
        }
    }
}
