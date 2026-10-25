use crate::display::{
    DISPLAY_APPEAR_TIMEOUT, DisplayDevice, DisplayError, POLL_INTERVAL, attached_display_devices,
    new_parsec_display, poll_until,
};

pub fn wait_for_new_parsec_display(
    before: &[DisplayDevice],
) -> Result<DisplayDevice, DisplayError> {
    poll_until(DISPLAY_APPEAR_TIMEOUT, POLL_INTERVAL, || {
        Ok::<_, DisplayError>(new_parsec_display(&attached_display_devices(), before).cloned())
    })?
    .ok_or(DisplayError::DisplayDidNotAppear)
}
