use super::{RunningSession, TEST_NONCE, TEST_SESSION, bind_peer, loopback_stream_socket};
use crate::hub::{Hub, ViewerRequest};
use crate::stream::{SessionControl, run_session};
use std::sync::Arc;
use std::thread;

pub fn start_session(hub: &Arc<Hub>, request: ViewerRequest) -> RunningSession {
    let socket = Arc::new(loopback_stream_socket());
    let peer = bind_peer();
    let address = peer.local_addr().expect("a bound socket has an address");
    let control = Arc::new(SessionControl::new());
    let thread = thread::spawn({
        let hub = hub.clone();
        let control = control.clone();

        move || {
            run_session(
                socket,
                address,
                TEST_SESSION,
                TEST_NONCE,
                hub,
                request,
                control,
            )
        }
    });

    RunningSession {
        peer,
        control,
        thread,
    }
}
