use crate::display::{
    DISPLAY_APPEAR_TIMEOUT, DisplayMode, FALLBACK_MODE, OUTPUT_APPEAR_TIMEOUT,
    PARSEC_VDD_DOWNLOAD_URL,
};
use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum DisplayError {
    Unsupported,
    DriverMissing,
    System {
        call: &'static str,
        source: io::Error,
    },
    DriverRequest {
        code: u32,
        source: io::Error,
    },
    DriverRefusedToAdd {
        reported: u32,
    },
    DisplayDidNotAppear,
    NoUsableMode {
        requested: DisplayMode,
    },
    ModeChangeRefused {
        device: String,
        mode: DisplayMode,
        code: i32,
    },
    DetachRefused {
        device: String,
        code: i32,
    },
    OutputNotFound {
        device: String,
    },
}

impl fmt::Display for DisplayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => {
                write!(formatter, "virtual displays are only available on Windows")
            }
            Self::DriverMissing => write!(
                formatter,
                "the Parsec Virtual Display Adapter driver is not installed: get Parsec VDD from {PARSEC_VDD_DOWNLOAD_URL}"
            ),
            Self::System { call, source } => write!(formatter, "{call} failed: {source}"),
            Self::DriverRequest { code, source } => write!(
                formatter,
                "the Parsec driver failed request {code:#010x}: {source}"
            ),
            Self::DriverRefusedToAdd { reported } => write!(
                formatter,
                "the Parsec driver did not add a virtual display (it answered {reported})"
            ),
            Self::DisplayDidNotAppear => write!(
                formatter,
                "the new virtual display did not show up within {} seconds",
                DISPLAY_APPEAR_TIMEOUT.as_secs()
            ),
            Self::NoUsableMode { requested } => write!(
                formatter,
                "the virtual display offers no mode of {}x{} and not even {FALLBACK_MODE}",
                requested.width, requested.height
            ),
            Self::ModeChangeRefused { device, mode, code } => write!(
                formatter,
                "Windows refused to switch {device} to {mode} (display change code {code})"
            ),
            Self::DetachRefused { device, code } => write!(
                formatter,
                "Windows refused to turn {device} off (display change code {code})"
            ),
            Self::OutputNotFound { device } => write!(
                formatter,
                "{device} did not become an output of the first DXGI adapter within {} seconds, so ddagrab cannot capture it",
                OUTPUT_APPEAR_TIMEOUT.as_secs()
            ),
        }
    }
}

impl Error for DisplayError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::System { source, .. } | Self::DriverRequest { source, .. } => Some(source),
            _ => None,
        }
    }
}
