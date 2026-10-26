use crate::link::receive_buffer_of;
use crate::link::test::opened_link;
use std::net::UdpSocket;

#[test]
fn a_link_asks_for_a_larger_receive_buffer_than_a_plain_socket_has() {
    let (_server, link) = opened_link();
    let plain = UdpSocket::bind("127.0.0.1:0").unwrap();

    assert!(
        link.receive_buffer_bytes() > receive_buffer_of(&plain),
        "the link has {} bytes, a plain socket {}",
        link.receive_buffer_bytes(),
        receive_buffer_of(&plain)
    );
}
