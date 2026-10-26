use crate::link::{RECEIVE_BUFFER_OPTION, SOCKET_LEVEL, raw_socket_of};
use std::net::UdpSocket;

pub(crate) fn request_receive_buffer(socket: &UdpSocket, bytes: u32) -> bool {
    let bytes = libc::c_int::try_from(bytes).unwrap_or(libc::c_int::MAX);

    let status = unsafe {
        libc::setsockopt(
            raw_socket_of(socket),
            SOCKET_LEVEL,
            RECEIVE_BUFFER_OPTION,
            (&raw const bytes).cast(),
            size_of::<libc::c_int>() as _,
        )
    };

    status == 0
}
