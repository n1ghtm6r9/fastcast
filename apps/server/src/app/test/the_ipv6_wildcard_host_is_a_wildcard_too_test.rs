use crate::app::urls_to_open;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn the_ipv6_wildcard_host_is_a_wildcard_too() {
    let lan = Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 7)));

    let urls = urls_to_open("::", 8080, lan);

    assert_eq!(urls, vec!["http://10.0.0.7:8080", "http://localhost:8080"]);
}
