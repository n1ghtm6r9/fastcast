use crate::gpu::DesktopError;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_MODE_ROTATION, DXGI_MODE_ROTATION_IDENTITY, DXGI_MODE_ROTATION_UNSPECIFIED,
};

pub fn require_upright(rotation: DXGI_MODE_ROTATION) -> Result<(), DesktopError> {
    if rotation == DXGI_MODE_ROTATION_IDENTITY || rotation == DXGI_MODE_ROTATION_UNSPECIFIED {
        Ok(())
    } else {
        Err(DesktopError::Rotated)
    }
}
