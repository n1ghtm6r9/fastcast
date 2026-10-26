use crate::server::test::FAKE_ADDRESS;
use crate::stream::StreamError;
use std::io::{self, ErrorKind};

pub fn busy_port_error() -> StreamError {
    StreamError::Bind {
        address: FAKE_ADDRESS,
        source: io::Error::from(ErrorKind::AddrInUse),
    }
}
