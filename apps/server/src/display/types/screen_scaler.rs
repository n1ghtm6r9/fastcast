use crate::display::{DisplayError, ScaleLevels};

pub trait ScreenScaler: Send + Sync + 'static {
    fn levels(&self, output: u32) -> Result<ScaleLevels, DisplayError>;

    fn set_level(&self, output: u32, level: i32) -> Result<(), DisplayError>;
}
