use crate::discovery::DiscoveryError;
use std::io;

#[test]
fn a_bind_failure_names_the_udp_address_and_the_cause() {
    let failure = DiscoveryError::Bind {
        address: "0.0.0.0:3278".parse().expect("a literal socket address"),
        source: io::Error::other("boom"),
    };

    assert_eq!(
        failure.to_string(),
        "could not listen on udp 0.0.0.0:3278: boom"
    );
}
