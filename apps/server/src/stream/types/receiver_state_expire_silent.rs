use crate::stream::ReceiverState;
use std::net::SocketAddr;
use std::time::Instant;

impl ReceiverState {
    pub fn expire_silent(&mut self, now: Instant) {
        let timeout = self.options.session_timeout;
        let silent: Vec<SocketAddr> = self
            .sessions
            .iter()
            .filter(|(_, entry)| now.saturating_duration_since(entry.last_seen) > timeout)
            .map(|(peer, _)| *peer)
            .collect();

        for peer in silent {
            self.end_session(peer, "went silent");
        }
    }
}
