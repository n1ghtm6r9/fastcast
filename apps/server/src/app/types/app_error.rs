use crate::stream::StreamError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Stream(StreamError),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Stream(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Stream(error) => Some(error),
        }
    }
}

impl From<StreamError> for AppError {
    fn from(error: StreamError) -> Self {
        Self::Stream(error)
    }
}
