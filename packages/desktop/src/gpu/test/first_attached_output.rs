use crate::gpu::{OutputInfo, list_outputs};

pub fn first_attached_output() -> OutputInfo {
    list_outputs()
        .unwrap()
        .into_iter()
        .find(|output| output.attached)
        .unwrap()
}
