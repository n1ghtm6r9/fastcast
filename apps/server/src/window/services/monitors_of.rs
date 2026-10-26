use crate::capture::FIRST_ADAPTER_INDEX;
use crate::window::Monitor;
use desktop::OutputInfo;

pub fn monitors_of(outputs: &[OutputInfo]) -> Vec<Monitor> {
    outputs
        .iter()
        .filter(|output| output.adapter_index == FIRST_ADAPTER_INDEX)
        .map(|output| Monitor {
            index: output.output_index,
            name: output.device_name.trim_start_matches(r"\\.\").to_owned(),
            width: output.width,
            height: output.height,
        })
        .collect()
}
