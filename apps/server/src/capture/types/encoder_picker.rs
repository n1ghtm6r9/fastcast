use crate::capture::{AUTO_ENCODER_ORDER, PickerState};
use crate::settings::{EncoderChoice, VideoEncoder};
use std::sync::Mutex;

pub struct EncoderPicker {
    pub(crate) choice: EncoderChoice,
    pub(crate) order: &'static [VideoEncoder],
    pub(crate) state: Mutex<PickerState>,
}

impl EncoderPicker {
    pub fn new(choice: EncoderChoice) -> Self {
        Self::ordered(choice, &AUTO_ENCODER_ORDER)
    }
}
