use crate::display::{DisplayError, DisplaySource, device_name_of_output, display_source_named};

pub fn source_of_output(output: u32) -> Result<DisplaySource, DisplayError> {
    let device = device_name_of_output(output)?.ok_or(DisplayError::NoSuchOutput { output })?;

    display_source_named(&device)?.ok_or(DisplayError::InactiveDisplay { device })
}
