use crate::settings::VideoEncoder;

#[derive(Debug, Default)]
pub struct PickerState {
    pub resolved: Option<VideoEncoder>,
    pub unusable: Vec<VideoEncoder>,
}
