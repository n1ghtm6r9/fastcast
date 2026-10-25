use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum FlvError {
    InvalidHeader,
    Io(io::Error),
}

impl fmt::Display for FlvError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlvError::InvalidHeader => write!(
                formatter,
                "the stream does not start with a valid FLV header"
            ),
            FlvError::Io(error) => write!(formatter, "the FLV stream could not be read: {}", error),
        }
    }
}

impl Error for FlvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            FlvError::InvalidHeader => None,
            FlvError::Io(error) => Some(error),
        }
    }
}

impl From<io::Error> for FlvError {
    fn from(error: io::Error) -> Self {
        FlvError::Io(error)
    }
}
