use crate::gpu::{
    DesktopError, OutputInfo, api_error, enumerate_all, list_adapters, output_info_of,
};

pub fn list_outputs() -> Result<Vec<OutputInfo>, DesktopError> {
    let mut outputs = Vec::new();

    for adapter in list_adapters()? {
        let adapter_description =
            unsafe { adapter.GetDesc1() }.map_err(api_error("IDXGIAdapter1::GetDesc1"))?;
        let adapter_outputs = enumerate_all("IDXGIAdapter::EnumOutputs", |index| unsafe {
            adapter.EnumOutputs(index)
        })?;

        for output in adapter_outputs {
            let output_description =
                unsafe { output.GetDesc() }.map_err(api_error("IDXGIOutput::GetDesc"))?;

            outputs.push(output_info_of(&adapter_description, &output_description));
        }
    }

    Ok(outputs)
}
