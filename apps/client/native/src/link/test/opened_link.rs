use crate::link::Link;
use crate::link::test::{FakeServer, test_target};

pub(crate) fn opened_link() -> (FakeServer, Link) {
    let server = FakeServer::new();
    let link = Link::open(test_target(server.address())).unwrap();

    (server, link)
}
