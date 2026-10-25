use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum WebError {
    Bind { address: String, source: io::Error },
    Serve(io::Error),
}

impl fmt::Display for WebError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind { address, source } => write!(
                formatter,
                "could not listen on {address}: {source} (is another instance already running?)"
            ),
            Self::Serve(source) => write!(formatter, "the web server stopped: {source}"),
        }
    }
}

impl Error for WebError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Bind { source, .. } | Self::Serve(source) => Some(source),
        }
    }
}
