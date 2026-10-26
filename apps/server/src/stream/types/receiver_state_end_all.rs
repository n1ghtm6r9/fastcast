use crate::stream::{ReceiverState, join_session_thread};
use std::mem::take;
use std::net::SocketAddr;

impl ReceiverState {
    pub fn end_all(&mut self) {
        let peers: Vec<SocketAddr> = self.sessions.keys().copied().collect();

        for peer in peers {
            self.end_session(peer, "server stopping");
        }

        for thread in take(&mut self.retired) {
            join_session_thread(thread);
        }
    }
}
