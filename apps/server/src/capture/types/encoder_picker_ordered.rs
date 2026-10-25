use crate::capture::{EncoderPicker, PickerState};
use crate::settings::{EncoderChoice, VideoEncoder};
use std::sync::Mutex;

impl EncoderPicker {
    pub fn ordered(choice: EncoderChoice, order: &'static [VideoEncoder]) -> Self {
        Self {
            choice,
            order,
            state: Mutex::new(PickerState::default()),
        }
    }
}
