use crate::app::urls_to_open;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn a_loopback_host_offers_only_localhost() {
    let lan = Some(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 15)));

    assert_eq!(
        urls_to_open("127.0.0.1", 3278, lan),
        vec!["http://localhost:3278"]
    );
    assert_eq!(
        urls_to_open("::1", 3278, lan),
        vec!["http://localhost:3278"]
    );
}
