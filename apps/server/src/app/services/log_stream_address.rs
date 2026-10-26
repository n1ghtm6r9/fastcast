use std::net::SocketAddr;
use tracing::info;

pub fn log_stream_address(address: SocketAddr) {
    info!("fastcast is listening on udp {address}, the TV finds it by itself");
}
