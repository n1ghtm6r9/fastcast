use crate::gpu::{AdapterLuid, DesktopError, GpuDevice};

#[test]
fn no_device_opens_on_an_unknown_adapter() {
    let unknown = AdapterLuid {
        low: 0xDEAD_BEEF,
        high: 0x0BAD_F00D,
    };

    assert!(matches!(
        GpuDevice::new(unknown),
        Err(DesktopError::AdapterNotFound)
    ));
}
