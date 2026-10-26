use crate::link::{
    LINK_TICK, Link, LinkShared, LinkTarget, SOCKET_BUFFER_REQUEST_BYTES, link_thread,
    receive_buffer_of, request_receive_buffer, unspecified_address_for,
};
use std::io;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;

impl Link {
    pub fn open(target: LinkTarget) -> io::Result<Link> {
        let socket = UdpSocket::bind(unspecified_address_for(&target.server))?;
        socket.set_read_timeout(Some(LINK_TICK))?;

        request_receive_buffer(&socket, SOCKET_BUFFER_REQUEST_BYTES);

        let receive_buffer_bytes = receive_buffer_of(&socket);
        let socket = Arc::new(socket);
        let shared = Arc::new(LinkShared::default());
        let server = target.server;
        let thread = thread::Builder::new()
            .name("fastcast-link".to_string())
            .spawn({
                let socket = Arc::clone(&socket);
                let shared = Arc::clone(&shared);
                move || link_thread(socket, target, shared)
            })?;

        Ok(Link {
            socket,
            server,
            shared,
            receive_buffer_bytes,
            thread: Mutex::new(Some(thread)),
        })
    }
}
