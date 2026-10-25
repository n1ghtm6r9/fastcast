use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

pub async fn next_datagram(socket: &UdpSocket) -> (String, SocketAddr) {
    let mut buffer = [0u8; 2048];

    let (length, sender) = timeout(Duration::from_secs(5), socket.recv_from(&mut buffer))
        .await
        .expect("a datagram arrives in time")
        .expect("the datagram is received");

    (
        String::from_utf8(buffer[..length].to_vec()).expect("the datagram is text"),
        sender,
    )
}
