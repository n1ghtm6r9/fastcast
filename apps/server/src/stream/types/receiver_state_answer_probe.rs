use crate::stream::ReceiverState;
use std::net::SocketAddr;
use tracing::debug;

impl ReceiverState {
    pub(crate) fn answer_probe(&self, peer: SocketAddr) {
        match self.socket.send_to(self.reply.as_bytes(), peer) {
            Ok(_) => debug!(%peer, "answered a discovery probe"),
            Err(error) => debug!(%peer, "could not answer a discovery probe: {error}"),
        }
    }
}
