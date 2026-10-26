use crate::stream::ReceiverState;
use std::net::SocketAddr;
use tracing::info;

impl ReceiverState {
    pub(crate) fn end_session(&mut self, peer: SocketAddr, why: &str) {
        let Some(entry) = self.sessions.remove(&peer) else {
            return;
        };

        entry.control.stop();
        self.retired.push(entry.thread);

        info!("viewer {peer} {why} (session {:08x})", entry.id);
    }
}
