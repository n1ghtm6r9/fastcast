use crate::display::{DisplayError, DisplayMode, VirtualDisplay};

impl VirtualDisplay {
    pub fn open(_requested: DisplayMode) -> Result<VirtualDisplay, DisplayError> {
        Err(DisplayError::Unsupported)
    }
}
