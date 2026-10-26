#[cfg(windows)]
use crate::capture::REOPEN_GIVE_UP_AFTER;
use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum NativeError {
    #[cfg(windows)]
    Desktop(desktop::DesktopError),
    #[cfg(windows)]
    Encoder(nvenc::NvencError),
    #[cfg(windows)]
    ReopenGaveUp(desktop::DesktopError),
    OutputMissing {
        index: u32,
    },
    NotNvidia {
        adapter: String,
    },
    UndescribableStream,
    Spawn(io::Error),
    WorkerGone,
    #[cfg(not(windows))]
    Unsupported,
}

impl fmt::Display for NativeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(windows)]
            Self::Desktop(error) => write!(formatter, "{error}"),
            #[cfg(windows)]
            Self::Encoder(error) => write!(formatter, "{error}"),
            #[cfg(windows)]
            Self::ReopenGaveUp(error) => write!(
                formatter,
                "the desktop could not be duplicated again for {} seconds: {error}",
                REOPEN_GIVE_UP_AFTER.as_secs()
            ),
            Self::OutputMissing { index } => write!(
                formatter,
                "output {index} of the first graphics adapter is not there"
            ),
            Self::NotNvidia { adapter } => write!(
                formatter,
                "the output is driven by {adapter}, not by an NVIDIA GPU"
            ),
            Self::UndescribableStream => write!(
                formatter,
                "the encoder wrote a key frame without parameter sets the server can describe"
            ),
            Self::Spawn(source) => {
                write!(formatter, "could not start the capture thread: {source}")
            }
            Self::WorkerGone => write!(formatter, "the capture thread stopped without a result"),
            #[cfg(not(windows))]
            Self::Unsupported => write!(formatter, "native capture needs Windows"),
        }
    }
}

impl Error for NativeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            #[cfg(windows)]
            Self::Desktop(error) => Some(error),
            #[cfg(windows)]
            Self::Encoder(error) => Some(error),
            #[cfg(windows)]
            Self::ReopenGaveUp(error) => Some(error),
            Self::Spawn(source) => Some(source),
            _ => None,
        }
    }
}
