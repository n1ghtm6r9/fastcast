use crate::display::{DisplayError, system_error, text_of_wide};
use windows::Win32::Graphics::Dxgi::IDXGIOutput;

pub fn output_device_name(output: &IDXGIOutput) -> Result<String, DisplayError> {
    let description = unsafe { output.GetDesc() }.map_err(system_error("IDXGIOutput::GetDesc"))?;

    Ok(text_of_wide(&description.DeviceName))
}
