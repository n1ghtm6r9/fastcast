use crate::support::receive_within;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub fn next_reply(prober: &UdpSocket, timeout: Duration) -> Option<(String, SocketAddr)> {
    let (bytes, sender) = receive_within(prober, timeout)?;

    Some((
        String::from_utf8(bytes).expect("a discovery reply is text"),
        sender,
    ))
}
