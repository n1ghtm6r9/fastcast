use crate::display::{
    DisplayError, OUTPUT_APPEAR_TIMEOUT, POLL_INTERVAL, find_output_index, poll_until,
};

pub fn wait_for_output_index(device_name: &str) -> Result<u32, DisplayError> {
    poll_until(OUTPUT_APPEAR_TIMEOUT, POLL_INTERVAL, || {
        find_output_index(device_name)
    })?
    .ok_or_else(|| DisplayError::OutputNotFound {
        device: device_name.to_owned(),
    })
}
