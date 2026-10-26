use crate::capture::{AUTO_ENCODER_ORDER, PickerState, VideoEncoder};
use std::sync::Mutex;

pub struct EncoderPicker {
    pub(crate) order: &'static [VideoEncoder],
    pub(crate) state: Mutex<PickerState>,
}

impl Default for EncoderPicker {
    fn default() -> Self {
        Self::ordered(&AUTO_ENCODER_ORDER)
    }
}
