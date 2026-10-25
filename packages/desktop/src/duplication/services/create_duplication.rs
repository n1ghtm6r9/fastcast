use crate::duplication::{DUPLICATION_FORMATS, duplication_error};
use crate::gpu::{DesktopError, GpuDevice, api_error};
use windows::Win32::Graphics::Dxgi::{
    IDXGIOutput, IDXGIOutput1, IDXGIOutput5, IDXGIOutputDuplication,
};
use windows::core::Interface;

pub fn create_duplication(
    device: &GpuDevice,
    output: &IDXGIOutput,
) -> Result<IDXGIOutputDuplication, DesktopError> {
    let with_formats = output.cast::<IDXGIOutput5>().and_then(|output5| unsafe {
        output5.DuplicateOutput1(device.device(), 0, &DUPLICATION_FORMATS)
    });

    if let Ok(duplication) = with_formats {
        return Ok(duplication);
    }

    let output1: IDXGIOutput1 = output
        .cast()
        .map_err(api_error("IDXGIOutput::QueryInterface"))?;

    unsafe { output1.DuplicateOutput(device.device()) }
        .map_err(|error| duplication_error("IDXGIOutput1::DuplicateOutput", error))
}
