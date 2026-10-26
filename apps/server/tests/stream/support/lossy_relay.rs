use crate::support::{POLL_STEP, RelayGuard, run_relay};
use app::stream::bind_stream_socket;
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::spawn;

pub fn lossy_relay(server: SocketAddr, drop_every: usize) -> RelayGuard {
    let socket =
        bind_stream_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("a free port exists");

    socket
        .set_read_timeout(Some(POLL_STEP))
        .expect("a read timeout is accepted");

    let local = socket.local_addr().expect("the relay has an address");
    let stop = Arc::new(AtomicBool::new(false));
    let thread = spawn({
        let stop = stop.clone();

        move || run_relay(socket, server, drop_every, stop)
    });

    RelayGuard {
        local,
        stop,
        thread: Some(thread),
    }
}
