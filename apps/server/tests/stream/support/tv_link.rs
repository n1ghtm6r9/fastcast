use crate::support::a_bare_hello;
use fastcast_native::{Link, LinkTarget};
use std::net::SocketAddr;

pub fn tv_link(server: SocketAddr) -> Link {
    Link::open(LinkTarget {
        server,
        hello: a_bare_hello(),
    })
    .expect("the tv link opens")
}
