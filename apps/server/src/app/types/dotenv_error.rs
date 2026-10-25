use std::error::Error;
use std::path::PathBuf;
use std::{fmt, io};

#[derive(Debug)]
pub enum DotenvError {
    Read {
        path: PathBuf,
        source: io::Error,
    },
    NotText {
        path: PathBuf,
    },
    Line {
        path: PathBuf,
        number: usize,
        text: String,
    },
}

impl fmt::Display for DotenvError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(formatter, "could not read {}: {source}", path.display())
            }
            Self::NotText { path } => write!(
                formatter,
                "{} is not UTF-8 or UTF-16 text, save it as UTF-8",
                path.display()
            ),
            Self::Line { path, number, text } => write!(
                formatter,
                "{} line {number} cannot be parsed: \"{text}\" \
                 (write NAME=value, and put a value with spaces or backslashes in single quotes)",
                path.display()
            ),
        }
    }
}

impl Error for DotenvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::NotText { .. } | Self::Line { .. } => None,
        }
    }
}
