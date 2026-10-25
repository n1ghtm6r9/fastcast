use crate::duplication::require_upright;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_MODE_ROTATION_IDENTITY, DXGI_MODE_ROTATION_UNSPECIFIED,
};

#[test]
fn an_unrotated_output_is_accepted() {
    assert!(require_upright(DXGI_MODE_ROTATION_IDENTITY).is_ok());
    assert!(require_upright(DXGI_MODE_ROTATION_UNSPECIFIED).is_ok());
}
