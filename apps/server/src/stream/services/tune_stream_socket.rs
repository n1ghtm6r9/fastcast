use crate::stream::SEND_BUFFER_BYTES;
use std::io;
use std::net::UdpSocket;
use std::os::windows::io::AsRawSocket;
use windows::Win32::Networking::WinSock::{
    SIO_UDP_CONNRESET, SO_SNDBUF, SOCKET, SOCKET_ERROR, SOL_SOCKET, WSAIoctl, setsockopt,
};

pub fn tune_stream_socket(socket: &UdpSocket) -> io::Result<()> {
    let handle = SOCKET(socket.as_raw_socket() as usize);
    let report_resets = 0u32;
    let mut returned = 0u32;

    let reset_status = unsafe {
        WSAIoctl(
            handle,
            SIO_UDP_CONNRESET,
            Some((&raw const report_resets).cast()),
            size_of_val(&report_resets) as u32,
            None,
            0,
            &mut returned,
            None,
            None,
        )
    };

    if reset_status == SOCKET_ERROR {
        return Err(io::Error::last_os_error());
    }

    let buffer_status = unsafe {
        setsockopt(
            handle,
            SOL_SOCKET,
            SO_SNDBUF,
            Some(&SEND_BUFFER_BYTES.to_ne_bytes()),
        )
    };

    if buffer_status == SOCKET_ERROR {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
