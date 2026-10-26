use crate::link::is_quiet_read_error;
use std::io;
use std::net::{SocketAddr, UdpSocket};

pub(crate) fn read_datagram(
    socket: &UdpSocket,
    buffer: &mut [u8],
) -> io::Result<(usize, SocketAddr)> {
    if cfg!(windows)
        && let Err(error) = socket.peek_from(buffer)
        && is_quiet_read_error(&error)
    {
        return Err(error);
    }

    socket.recv_from(buffer)
}
