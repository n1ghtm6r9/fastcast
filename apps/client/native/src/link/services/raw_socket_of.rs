use std::net::UdpSocket;
#[cfg(unix)]
use std::os::fd::AsRawFd;
#[cfg(windows)]
use std::os::windows::io::AsRawSocket;

#[cfg(unix)]
pub(crate) fn raw_socket_of(socket: &UdpSocket) -> libc::c_int {
    socket.as_raw_fd()
}

#[cfg(windows)]
pub(crate) fn raw_socket_of(socket: &UdpSocket) -> libc::SOCKET {
    socket.as_raw_socket() as libc::SOCKET
}
