use crate::gpu::{OutputInfo, adapter_luid_of, desktop_extent, wide_to_string};
use windows::Win32::Graphics::Dxgi::{DXGI_ADAPTER_DESC1, DXGI_OUTPUT_DESC};

pub fn output_info_of(adapter: &DXGI_ADAPTER_DESC1, output: &DXGI_OUTPUT_DESC) -> OutputInfo {
    let area = output.DesktopCoordinates;

    OutputInfo {
        device_name: wide_to_string(&output.DeviceName),
        adapter_name: wide_to_string(&adapter.Description),
        adapter_luid: adapter_luid_of(adapter.AdapterLuid),
        vendor_id: adapter.VendorId,
        left: area.left,
        top: area.top,
        width: desktop_extent(area.left, area.right),
        height: desktop_extent(area.top, area.bottom),
        attached: output.AttachedToDesktop.as_bool(),
    }
}
