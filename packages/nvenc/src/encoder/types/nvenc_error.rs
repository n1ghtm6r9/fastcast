use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NvencError {
    LibraryMissing,
    EntryPointMissing,
    DriverTooOld,
    CodecUnsupported,
    InvalidSettings,
    OutOfMemory,
    TextureLimit,
    TextureMismatch,
    Status { call: &'static str, status: i32 },
}

impl fmt::Display for NvencError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NvencError::LibraryMissing => write!(
                formatter,
                "nvEncodeAPI64.dll was not found; install the NVIDIA driver"
            ),
            NvencError::EntryPointMissing => write!(
                formatter,
                "nvEncodeAPI64.dll does not provide the NVENC entry points; reinstall the NVIDIA driver"
            ),
            NvencError::DriverTooOld => write!(
                formatter,
                "the NVIDIA driver is older than NVENC API 13.0; update the driver"
            ),
            NvencError::CodecUnsupported => write!(
                formatter,
                "this GPU cannot encode the requested codec at the requested bit depth"
            ),
            NvencError::InvalidSettings => write!(
                formatter,
                "the encoder settings are outside the supported ranges"
            ),
            NvencError::OutOfMemory => write!(formatter, "the encoder ran out of memory"),
            NvencError::TextureLimit => write!(
                formatter,
                "an encoder accepts at most 4 different input textures"
            ),
            NvencError::TextureMismatch => write!(
                formatter,
                "the input texture does not match the encoder size and format"
            ),
            NvencError::Status { call, status } => {
                write!(formatter, "{} failed with NVENCSTATUS {}", call, status)
            }
        }
    }
}

impl Error for NvencError {}
