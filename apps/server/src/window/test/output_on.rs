use desktop::{AdapterLuid, OutputInfo};

pub fn output_on(
    adapter_index: u32,
    output_index: u32,
    device_name: &str,
    width: u32,
    height: u32,
) -> OutputInfo {
    OutputInfo {
        device_name: device_name.to_owned(),
        adapter_name: "NVIDIA GeForce RTX 5070".to_owned(),
        adapter_luid: AdapterLuid { low: 1, high: 0 },
        vendor_id: 0x10de,
        left: 0,
        top: 0,
        width,
        height,
        attached: true,
        adapter_index,
        output_index,
    }
}
