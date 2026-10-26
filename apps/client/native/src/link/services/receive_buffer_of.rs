use crate::link::{RECEIVE_BUFFER_OPTION, SOCKET_LEVEL, raw_socket_of};
use std::net::UdpSocket;

pub(crate) fn receive_buffer_of(socket: &UdpSocket) -> u32 {
    let mut bytes: libc::c_int = 0;
    let mut length = size_of::<libc::c_int>() as _;

    let status = unsafe {
        libc::getsockopt(
            raw_socket_of(socket),
            SOCKET_LEVEL,
            RECEIVE_BUFFER_OPTION,
            (&raw mut bytes).cast(),
            &mut length,
        )
    };

    if status == 0 {
        u32::try_from(bytes).unwrap_or(0)
    } else {
        0
    }
}
