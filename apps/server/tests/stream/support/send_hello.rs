use crate::support::encoded_datagram;
use std::net::{SocketAddr, UdpSocket};
use wire::{Hello, Packet};

pub fn send_hello(socket: &UdpSocket, server: SocketAddr, hello: &Hello) {
    socket
        .send_to(
            &encoded_datagram(0, 0, &Packet::Hello(hello.clone())),
            server,
        )
        .expect("the hello is sent");
}
