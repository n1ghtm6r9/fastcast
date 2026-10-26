use crate::stream::RECEIVE_BUFFER_BYTES;
use std::io::ErrorKind;
use std::net::UdpSocket;

pub fn next_datagram_bytes(peer: &UdpSocket) -> Option<Vec<u8>> {
    let mut buffer = [0u8; RECEIVE_BUFFER_BYTES];

    match peer.recv_from(&mut buffer) {
        Ok((length, _)) => Some(buffer[..length].to_vec()),
        Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => None,
        Err(error) => panic!("the peer socket failed: {error}"),
    }
}
