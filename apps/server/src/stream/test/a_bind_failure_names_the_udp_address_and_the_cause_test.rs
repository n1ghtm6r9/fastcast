use crate::stream::StreamError;
use std::io::{self, ErrorKind};

#[test]
fn a_bind_failure_names_the_udp_address_and_the_cause() {
    let failure = StreamError::Bind {
        address: "0.0.0.0:3278".parse().expect("a literal socket address"),
        source: io::Error::new(ErrorKind::AddrInUse, "taken"),
    };

    assert!(
        failure
            .to_string()
            .starts_with("could not listen on udp 0.0.0.0:3278: taken")
    );
}
