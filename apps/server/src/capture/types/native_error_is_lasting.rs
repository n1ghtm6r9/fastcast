use crate::capture::NativeError;
#[cfg(windows)]
use desktop::DesktopError;
#[cfg(windows)]
use nvenc::NvencError;

impl NativeError {
    pub fn is_lasting(&self) -> bool {
        match self {
            #[cfg(windows)]
            Self::Desktop(error) => matches!(error, DesktopError::ShaderCompile(_)),
            #[cfg(windows)]
            Self::Encoder(error) => matches!(
                error,
                NvencError::LibraryMissing
                    | NvencError::EntryPointMissing
                    | NvencError::DriverTooOld
                    | NvencError::CodecUnsupported
                    | NvencError::InvalidSettings
            ),
            Self::NotNvidia { .. } | Self::UndescribableStream => true,
            Self::OutputMissing { .. } | Self::Spawn(_) | Self::WorkerGone => false,
            #[cfg(windows)]
            Self::ReopenGaveUp(_) => false,
            #[cfg(not(windows))]
            Self::Unsupported => true,
        }
    }
}
