use crate::app::lan_ip;
use std::net::{IpAddr, Ipv4Addr};
use tracing::info;

pub fn log_stream_address(port: u16) {
    let ip = lan_ip().unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED));

    info!("fastcast is listening on udp {ip}:{port}, the TV finds it by itself");
}
