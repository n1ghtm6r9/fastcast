use crate::stream::{
    ReceiverState, SessionControl, SessionEntry, new_session_id, run_session, viewer_request_of,
};
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread::Builder;
use std::time::Instant;
use tracing::{info, warn};
use wire::{FecController, Hello, LossMeter};

impl ReceiverState {
    pub(crate) fn start_session(&mut self, peer: SocketAddr, hello: &Hello, now: Instant) {
        let previous = self.sessions.get(&peer).map_or(0, |entry| entry.id);

        self.end_session(peer, "replaced");

        let id = new_session_id(previous);
        let control = Arc::new(SessionControl::new());
        let request = viewer_request_of(hello, self.options.default_fps);
        let spawned = Builder::new().name("fastcast-session".to_owned()).spawn({
            let socket = self.socket.clone();
            let hub = self.hub.clone();
            let control = control.clone();
            let request = request.clone();
            let nonce = hello.nonce;

            move || run_session(socket, peer, id, nonce, hub, request, control)
        });

        match spawned {
            Ok(thread) => {
                let mode = request
                    .mode
                    .map_or_else(|| "any".to_owned(), |mode| mode.to_string());

                info!(
                    "viewer {peer} joined as session {id:08x}, mode {mode}, codecs {:?}",
                    request.codecs
                );

                self.sessions.insert(
                    peer,
                    SessionEntry {
                        id,
                        nonce: hello.nonce,
                        last_seen: now,
                        control,
                        loss: LossMeter::new(),
                        fec: FecController::new(now),
                        thread,
                    },
                );
            }
            Err(error) => warn!(%peer, "could not start a stream session: {error}"),
        }
    }
}
