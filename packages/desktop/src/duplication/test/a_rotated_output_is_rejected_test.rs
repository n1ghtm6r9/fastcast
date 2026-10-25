use crate::duplication::require_upright;
use crate::gpu::DesktopError;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_MODE_ROTATION_ROTATE90, DXGI_MODE_ROTATION_ROTATE180, DXGI_MODE_ROTATION_ROTATE270,
};

#[test]
fn a_rotated_output_is_rejected() {
    for rotation in [
        DXGI_MODE_ROTATION_ROTATE90,
        DXGI_MODE_ROTATION_ROTATE180,
        DXGI_MODE_ROTATION_ROTATE270,
    ] {
        assert!(matches!(
            require_upright(rotation),
            Err(DesktopError::Rotated)
        ));
    }
}
