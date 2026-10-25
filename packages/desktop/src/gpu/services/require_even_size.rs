use crate::gpu::DesktopError;

pub fn require_even_size(width: u32, height: u32) -> Result<(), DesktopError> {
    if width.is_multiple_of(2) && height.is_multiple_of(2) {
        Ok(())
    } else {
        Err(DesktopError::OddSize { width, height })
    }
}
