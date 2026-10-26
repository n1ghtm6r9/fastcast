use std::error::Error;
use std::net::SocketAddr;
use std::{fmt, io};

#[derive(Debug)]
pub enum StreamError {
    Bind {
        address: SocketAddr,
        source: io::Error,
    },
    Socket {
        source: io::Error,
    },
}

impl fmt::Display for StreamError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind { address, source } => write!(
                formatter,
                "could not listen on udp {address}: {source} (is another FastCast already running?)"
            ),
            Self::Socket { source } => {
                write!(formatter, "could not set up the stream socket: {source}")
            }
        }
    }
}

impl Error for StreamError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Bind { source, .. } | Self::Socket { source } => Some(source),
        }
    }
}
