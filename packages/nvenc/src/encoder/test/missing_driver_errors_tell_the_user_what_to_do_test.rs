use crate::encoder::NvencError;

#[test]
fn missing_driver_errors_tell_the_user_what_to_do() {
    assert_eq!(
        NvencError::LibraryMissing.to_string(),
        "nvEncodeAPI64.dll was not found; install the NVIDIA driver"
    );
    assert_eq!(
        NvencError::DriverTooOld.to_string(),
        "the NVIDIA driver is older than NVENC API 13.0; update the driver"
    );
}
