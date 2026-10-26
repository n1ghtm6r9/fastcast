use crate::link::LinkTarget;
use crate::link::test::test_hello;
use std::net::SocketAddr;

pub(crate) fn test_target(server: SocketAddr) -> LinkTarget {
    LinkTarget {
        server,
        hello: test_hello(),
    }
}
