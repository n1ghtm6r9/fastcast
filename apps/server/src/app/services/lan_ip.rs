use std::net::{IpAddr, UdpSocket};

pub fn lan_ip() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;

    socket.connect("8.8.8.8:80").ok()?;

    socket.local_addr().ok().map(|address| address.ip())
}
