use crate::capture::{
    priority_line, raise_process_priority, raise_thread_priority, refuse_power_throttling,
};
use desktop::{GpuDevice, raise_gpu_scheduling};

pub fn raise_capture_priorities(device: &GpuDevice) -> String {
    priority_line(&[
        (
            "process above normal",
            raise_process_priority().map_err(|error| error.to_string()),
        ),
        (
            "capture thread highest",
            raise_thread_priority().map_err(|error| error.to_string()),
        ),
        (
            "power throttling refused",
            refuse_power_throttling().map_err(|error| error.to_string()),
        ),
        (
            "GPU scheduling high",
            raise_gpu_scheduling().map_err(|error| error.to_string()),
        ),
        (
            "GPU threads highest",
            device.raise_priority().map_err(|error| error.to_string()),
        ),
    ])
}
