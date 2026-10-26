use crate::capture::{CaptureTarget, FIRST_ADAPTER_INDEX};
use desktop::OutputInfo;

pub fn native_output_for<'a>(
    target: &CaptureTarget,
    outputs: &'a [OutputInfo],
) -> Option<&'a OutputInfo> {
    outputs.iter().find(|output| {
        output.adapter_index == FIRST_ADAPTER_INDEX && output.output_index == target.output
    })
}
