use crate::encoder::{NvencError, nvenc_result};
use crate::ffi::FcResult;

#[test]
fn c_status_codes_map_to_nvenc_errors() {
    let mapped = |status, call| nvenc_result(FcResult { status, call });

    assert_eq!(mapped(0, 0), Ok(()));
    assert_eq!(mapped(-1, 1), Err(NvencError::LibraryMissing));
    assert_eq!(mapped(-2, 1), Err(NvencError::EntryPointMissing));
    assert_eq!(mapped(-3, 1), Err(NvencError::DriverTooOld));
    assert_eq!(mapped(-4, 3), Err(NvencError::CodecUnsupported));
    assert_eq!(mapped(-5, 0), Err(NvencError::InvalidSettings));
    assert_eq!(mapped(-6, 10), Err(NvencError::OutOfMemory));
    assert_eq!(mapped(-7, 8), Err(NvencError::TextureLimit));
    assert_eq!(
        mapped(8, 10),
        Err(NvencError::Status {
            call: "nvEncEncodePicture",
            status: 8
        })
    );
}
