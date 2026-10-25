use crate::app::urls_to_open;
use std::net::{IpAddr, Ipv4Addr};

#[test]
fn a_wildcard_host_offers_the_lan_address_and_localhost() {
    let lan = Some(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 15)));

    let urls = urls_to_open("0.0.0.0", 3278, lan);

    assert_eq!(
        urls,
        vec!["http://192.168.0.15:3278", "http://localhost:3278"]
    );
}
