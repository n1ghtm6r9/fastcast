use crate::display::{DisplayError, first_adapter, output_at, output_device_name};

pub fn device_name_of_output(index: u32) -> Result<Option<String>, DisplayError> {
    output_at(&first_adapter()?, index)?
        .map(|output| output_device_name(&output))
        .transpose()
}
