use crate::gpu::AdapterLuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputInfo {
    pub device_name: String,
    pub adapter_name: String,
    pub adapter_luid: AdapterLuid,
    pub vendor_id: u32,
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
    pub attached: bool,
}
