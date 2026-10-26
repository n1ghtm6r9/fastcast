use crate::link::{is_quiet_read_error, read_datagram};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

#[test]
fn a_reset_reported_by_the_system_is_read_only_once() {
    let closed = UdpSocket::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_millis(50)))
        .unwrap();
    socket.send_to(b"x", closed).unwrap();
    thread::sleep(Duration::from_millis(100));
    let mut buffer = [0u8; 64];

    let _ = read_datagram(&socket, &mut buffer);
    let second = read_datagram(&socket, &mut buffer).unwrap_err();

    assert!(is_quiet_read_error(&second));
}
