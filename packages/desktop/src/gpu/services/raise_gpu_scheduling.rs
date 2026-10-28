use crate::gpu::{DesktopError, api_error};
use windows::Wdk::Graphics::Direct3D::{
    D3DKMT_SCHEDULINGPRIORITYCLASS_HIGH, D3DKMTSetProcessSchedulingPriorityClass,
};
use windows::Win32::System::Threading::GetCurrentProcess;

pub fn raise_gpu_scheduling() -> Result<(), DesktopError> {
    unsafe {
        D3DKMTSetProcessSchedulingPriorityClass(
            GetCurrentProcess(),
            D3DKMT_SCHEDULINGPRIORITYCLASS_HIGH,
        )
    }
    .ok()
    .map_err(api_error("D3DKMTSetProcessSchedulingPriorityClass"))
}
