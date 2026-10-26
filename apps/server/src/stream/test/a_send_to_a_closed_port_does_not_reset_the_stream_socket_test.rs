use crate::stream::bind_stream_socket;
use std::io::ErrorKind;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::time::Duration;

#[test]
fn a_send_to_a_closed_port_does_not_reset_the_stream_socket() {
    let socket =
        bind_stream_socket(SocketAddr::from((Ipv4Addr::LOCALHOST, 0))).expect("a free port exists");

    socket
        .set_read_timeout(Some(Duration::from_millis(300)))
        .expect("a read timeout can be set");

    let vanished = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).expect("a free port exists");
    let closed = vanished
        .local_addr()
        .expect("a bound socket has an address");

    drop(vanished);

    socket
        .send_to(b"first", closed)
        .expect("the first send to a closed port succeeds");
    socket
        .send_to(b"second", closed)
        .expect("the second send to a closed port succeeds");

    let mut buffer = [0u8; 64];
    let failure = socket
        .recv_from(&mut buffer)
        .expect_err("nothing was sent to the socket");

    assert!(
        matches!(failure.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut),
        "the read failed with {failure:?}"
    );
}
