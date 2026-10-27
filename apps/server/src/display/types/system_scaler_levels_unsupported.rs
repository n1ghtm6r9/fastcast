use crate::display::{DisplayError, ScaleLevels, ScreenScaler, SystemScaler};

impl ScreenScaler for SystemScaler {
    fn levels(&self, _output: u32) -> Result<ScaleLevels, DisplayError> {
        Err(DisplayError::ScaleUnsupported)
    }

    fn set_level(&self, _output: u32, _level: i32) -> Result<(), DisplayError> {
        Err(DisplayError::ScaleUnsupported)
    }
}
