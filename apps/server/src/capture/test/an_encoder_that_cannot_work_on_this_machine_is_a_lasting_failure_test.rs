use crate::capture::NativeError;
use nvenc::NvencError;

#[test]
fn an_encoder_that_cannot_work_on_this_machine_is_a_lasting_failure() {
    for error in [
        NvencError::LibraryMissing,
        NvencError::EntryPointMissing,
        NvencError::DriverTooOld,
        NvencError::CodecUnsupported,
        NvencError::InvalidSettings,
    ] {
        assert!(NativeError::Encoder(error).is_lasting(), "{error}");
    }
}
