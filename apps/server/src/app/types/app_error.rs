use crate::app::DotenvError;
use crate::settings::SettingsError;
use crate::stream::StreamError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Dotenv(DotenvError),
    Settings(SettingsError),
    Stream(StreamError),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dotenv(error) => write!(formatter, "{error}"),
            Self::Settings(error) => write!(formatter, "invalid settings: {error}"),
            Self::Stream(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Dotenv(error) => Some(error),
            Self::Settings(error) => Some(error),
            Self::Stream(error) => Some(error),
        }
    }
}

impl From<DotenvError> for AppError {
    fn from(error: DotenvError) -> Self {
        Self::Dotenv(error)
    }
}

impl From<SettingsError> for AppError {
    fn from(error: SettingsError) -> Self {
        Self::Settings(error)
    }
}

impl From<StreamError> for AppError {
    fn from(error: StreamError) -> Self {
        Self::Stream(error)
    }
}
