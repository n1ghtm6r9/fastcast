use crate::display::{DisplayError, first_adapter, output_at, output_device_name};

pub fn find_output_index(device_name: &str) -> Result<Option<u32>, DisplayError> {
    let adapter = first_adapter()?;
    let mut index = 0;

    while let Some(output) = output_at(&adapter, index)? {
        if output_device_name(&output)?.eq_ignore_ascii_case(device_name) {
            return Ok(Some(index));
        }

        index += 1;
    }

    Ok(None)
}
