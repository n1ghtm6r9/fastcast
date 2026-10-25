use crate::settings::SettingsError;
use crate::web::WebError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Settings(SettingsError),
    Web(WebError),
}

impl fmt::Display for AppError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Settings(error) => write!(formatter, "invalid settings: {error}"),
            Self::Web(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Settings(error) => Some(error),
            Self::Web(error) => Some(error),
        }
    }
}

impl From<SettingsError> for AppError {
    fn from(error: SettingsError) -> Self {
        Self::Settings(error)
    }
}

impl From<WebError> for AppError {
    fn from(error: WebError) -> Self {
        Self::Web(error)
    }
}
