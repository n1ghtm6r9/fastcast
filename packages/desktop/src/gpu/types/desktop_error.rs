use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DesktopError {
    Api {
        call: &'static str,
        source: windows::core::Error,
    },
    AccessLost,
    AdapterNotFound,
    OddSize {
        width: u32,
        height: u32,
    },
    Rotated,
    ShaderCompile(String),
}

impl fmt::Display for DesktopError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DesktopError::Api { call, source } => {
                write!(formatter, "{} failed: {}", call, source)
            }
            DesktopError::AccessLost => write!(
                formatter,
                "access to the desktop was lost and the capture must be created again"
            ),
            DesktopError::AdapterNotFound => {
                write!(
                    formatter,
                    "no graphics adapter has the requested identifier"
                )
            }
            DesktopError::OddSize { width, height } => write!(
                formatter,
                "a picture of {}x{} pixels cannot be stored as 4:2:0 because a side is odd",
                width, height
            ),
            DesktopError::Rotated => write!(formatter, "a rotated output cannot be captured"),
            DesktopError::ShaderCompile(message) => {
                write!(formatter, "a shader did not compile: {}", message)
            }
        }
    }
}

impl Error for DesktopError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DesktopError::Api { source, .. } => Some(source),
            DesktopError::AccessLost
            | DesktopError::AdapterNotFound
            | DesktopError::OddSize { .. }
            | DesktopError::Rotated
            | DesktopError::ShaderCompile(_) => None,
        }
    }
}
