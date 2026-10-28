use crate::gpu::{DesktopError, TEMPORARY_DESKTOP_ERRORS};

impl DesktopError {
    pub fn is_temporary(&self) -> bool {
        match self {
            DesktopError::AccessLost => true,
            DesktopError::Api { source, .. } => TEMPORARY_DESKTOP_ERRORS.contains(&source.code()),
            DesktopError::AdapterNotFound
            | DesktopError::OddSize { .. }
            | DesktopError::Rotated
            | DesktopError::ShaderCompile(_) => false,
        }
    }
}
