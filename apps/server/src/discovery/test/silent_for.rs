use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;

pub async fn silent_for(socket: &UdpSocket, wait: Duration) -> bool {
    let mut buffer = [0u8; 2048];

    timeout(wait, socket.recv_from(&mut buffer)).await.is_err()
}
