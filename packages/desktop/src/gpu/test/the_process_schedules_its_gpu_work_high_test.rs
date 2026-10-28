use crate::gpu::raise_gpu_scheduling;
use windows::Wdk::Graphics::Direct3D::{
    D3DKMT_SCHEDULINGPRIORITYCLASS, D3DKMT_SCHEDULINGPRIORITYCLASS_HIGH,
    D3DKMTGetProcessSchedulingPriorityClass,
};
use windows::Win32::System::Threading::GetCurrentProcess;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn the_process_schedules_its_gpu_work_high() {
    raise_gpu_scheduling().unwrap();

    let mut class = D3DKMT_SCHEDULINGPRIORITYCLASS::default();
    unsafe { D3DKMTGetProcessSchedulingPriorityClass(GetCurrentProcess(), &mut class) }
        .ok()
        .unwrap();
    assert_eq!(class, D3DKMT_SCHEDULINGPRIORITYCLASS_HIGH);
}
