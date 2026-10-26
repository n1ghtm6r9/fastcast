use crate::capture::VideoEncoder;

#[derive(Debug, Default)]
pub struct PickerState {
    pub resolved: Option<VideoEncoder>,
    pub unusable: Vec<VideoEncoder>,
}
