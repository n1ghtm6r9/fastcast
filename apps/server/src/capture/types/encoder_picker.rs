use crate::capture::PickerState;
use crate::settings::EncoderChoice;
use std::sync::Mutex;

pub struct EncoderPicker {
    pub(crate) choice: EncoderChoice,
    pub(crate) state: Mutex<PickerState>,
}

impl EncoderPicker {
    pub fn new(choice: EncoderChoice) -> Self {
        Self {
            choice,
            state: Mutex::new(PickerState::default()),
        }
    }
}
