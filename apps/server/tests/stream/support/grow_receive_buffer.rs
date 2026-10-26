use crate::support::VIEWER_BUFFER_BYTES;
use std::net::UdpSocket;
use std::os::windows::io::AsRawSocket;
use windows::Win32::Networking::WinSock::{
    SO_RCVBUF, SOCKET, SOCKET_ERROR, SOL_SOCKET, setsockopt,
};

pub fn grow_receive_buffer(socket: &UdpSocket) {
    let handle = SOCKET(socket.as_raw_socket() as usize);
    let status = unsafe {
        setsockopt(
            handle,
            SOL_SOCKET,
            SO_RCVBUF,
            Some(&VIEWER_BUFFER_BYTES.to_ne_bytes()),
        )
    };

    assert_ne!(status, SOCKET_ERROR, "the receive buffer is resized");
}
