use crate::capture::{CaptureTarget, NativeError, native_output_for};
use desktop::{NVIDIA_VENDOR_ID, OutputInfo};

pub fn nvidia_output_for<'a>(
    target: &CaptureTarget,
    outputs: &'a [OutputInfo],
) -> Result<&'a OutputInfo, NativeError> {
    let output = native_output_for(target, outputs).ok_or(NativeError::OutputMissing {
        index: target.output,
    })?;

    if output.vendor_id != NVIDIA_VENDOR_ID {
        return Err(NativeError::NotNvidia {
            adapter: output.adapter_name.clone(),
        });
    }

    Ok(output)
}
