use crate::discovery::is_discovery_probe;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::debug;

pub async fn answer_datagram(socket: &UdpSocket, datagram: &[u8], sender: SocketAddr, reply: &str) {
    if !is_discovery_probe(datagram) {
        debug!(
            "ignored a datagram of {} bytes from {sender}",
            datagram.len()
        );
        return;
    }

    match socket.send_to(reply.as_bytes(), sender).await {
        Ok(_) => debug!("answered the discovery probe from {sender}"),
        Err(error) => debug!("could not answer the discovery probe from {sender}: {error}"),
    }
}
