use std::net::Ipv4Addr;
use tokio::net::UdpSocket;

pub async fn bind_prober() -> UdpSocket {
    UdpSocket::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .expect("a free port exists")
}
