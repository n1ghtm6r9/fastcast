use desktop::{AdapterLuid, NVIDIA_VENDOR_ID, OutputInfo};

pub fn numbered_output(adapter_index: u32, output_index: u32, device_name: &str) -> OutputInfo {
    OutputInfo {
        device_name: device_name.to_owned(),
        adapter_name: "a test adapter".to_owned(),
        adapter_luid: AdapterLuid { low: 1, high: 0 },
        vendor_id: NVIDIA_VENDOR_ID,
        left: 0,
        top: 0,
        width: 1920,
        height: 1080,
        attached: true,
        adapter_index,
        output_index,
    }
}
