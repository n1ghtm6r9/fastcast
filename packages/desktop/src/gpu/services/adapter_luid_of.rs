use crate::gpu::AdapterLuid;
use windows::Win32::Foundation::LUID;

pub fn adapter_luid_of(luid: LUID) -> AdapterLuid {
    AdapterLuid {
        low: luid.LowPart,
        high: luid.HighPart,
    }
}
