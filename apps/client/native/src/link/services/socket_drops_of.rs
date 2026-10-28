#[cfg(any(target_os = "linux", target_os = "android"))]
use crate::link::{
    MEMORY_INFO_FIELDS, MEMORY_INFO_OPTION, SOCKET_LEVEL, raw_socket_of, socket_drops_in,
};
use std::net::UdpSocket;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub(crate) fn socket_drops_of(socket: &UdpSocket) -> Option<u32> {
    let mut report = [0u32; MEMORY_INFO_FIELDS];
    let mut length = size_of_val(&report) as libc::socklen_t;

    let status = unsafe {
        libc::getsockopt(
            raw_socket_of(socket),
            SOCKET_LEVEL,
            MEMORY_INFO_OPTION,
            report.as_mut_ptr().cast(),
            &mut length,
        )
    };

    if status == 0 {
        socket_drops_in(&report, length as usize)
    } else {
        None
    }
}

#[cfg(not(any(target_os = "linux", target_os = "android")))]
pub(crate) fn socket_drops_of(_socket: &UdpSocket) -> Option<u32> {
    None
}
