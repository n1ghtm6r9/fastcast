use crate::discovery::DiscoveryError;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub async fn bind_discovery_socket(address: SocketAddr) -> Result<UdpSocket, DiscoveryError> {
    UdpSocket::bind(address)
        .await
        .map_err(|source| DiscoveryError::Bind { address, source })
}
