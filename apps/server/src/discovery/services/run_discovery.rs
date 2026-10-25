use crate::discovery::{bind_discovery_socket, serve_discovery};
use std::net::SocketAddr;
use tracing::{info, warn};

pub async fn run_discovery(address: SocketAddr, name: String, web_port: u16) {
    let socket = match bind_discovery_socket(address).await {
        Ok(socket) => socket,
        Err(error) => {
            warn!("discovery is off: {error}");
            return;
        }
    };

    info!("answering discovery on udp {address} as {name}");

    serve_discovery(socket, name, web_port).await;
}
