use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub const TEST_ADDRESS: SocketAddr =
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 10)), 3278);
