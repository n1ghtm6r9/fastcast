use crate::link::socket_drops_of;
use std::net::UdpSocket;

#[test]
fn a_socket_on_a_system_without_a_drop_count_reports_none() {
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    assert_eq!(socket_drops_of(&socket), None);
}
