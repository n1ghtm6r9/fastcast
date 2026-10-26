use crate::capture::{EncoderPicker, PickerState, VideoEncoder};
use std::sync::Mutex;

impl EncoderPicker {
    pub fn ordered(order: &'static [VideoEncoder]) -> Self {
        Self {
            order,
            state: Mutex::new(PickerState::default()),
        }
    }
}
