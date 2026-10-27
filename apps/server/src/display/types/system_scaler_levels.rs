use crate::display::{
    DisplayError, ScaleLevels, ScreenScaler, SystemScaler, scale_levels_of, set_scale_level_of,
    source_of_output,
};

impl ScreenScaler for SystemScaler {
    fn levels(&self, output: u32) -> Result<ScaleLevels, DisplayError> {
        scale_levels_of(source_of_output(output)?)
    }

    fn set_level(&self, output: u32, level: i32) -> Result<(), DisplayError> {
        set_scale_level_of(source_of_output(output)?, level)
    }
}
