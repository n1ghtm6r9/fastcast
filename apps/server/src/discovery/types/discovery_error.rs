use std::error::Error;
use std::net::SocketAddr;
use std::{fmt, io};

#[derive(Debug)]
pub enum DiscoveryError {
    Bind {
        address: SocketAddr,
        source: io::Error,
    },
}

impl fmt::Display for DiscoveryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind { address, source } => {
                write!(formatter, "could not listen on udp {address}: {source}")
            }
        }
    }
}

impl Error for DiscoveryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Bind { source, .. } => Some(source),
        }
    }
}
