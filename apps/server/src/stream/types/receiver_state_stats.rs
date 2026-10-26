use crate::stream::{ReceiverState, VIEWER_REPORT_LIMIT, clipped};
use std::net::SocketAddr;
use std::time::Instant;
use tracing::info;

impl ReceiverState {
    pub(crate) fn stats(&mut self, peer: SocketAddr, session: u32, text: &str, now: Instant) {
        let Some(entry) = self
            .sessions
            .get_mut(&peer)
            .filter(|entry| entry.id == session)
        else {
            self.refuse_stranger(peer, session);

            return;
        };

        entry.last_seen = now;

        info!("viewer report: {}", clipped(text, VIEWER_REPORT_LIMIT));
    }
}
