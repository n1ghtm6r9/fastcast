use crate::capture::NativeError;
use nvenc::NvencError;

#[test]
fn an_encoder_that_failed_for_now_is_not_a_lasting_failure() {
    for error in [
        NvencError::OutOfMemory,
        NvencError::TextureLimit,
        NvencError::TextureMismatch,
        NvencError::Status {
            call: "nvEncEncodePicture",
            status: 8,
        },
    ] {
        assert!(!NativeError::Encoder(error).is_lasting(), "{error}");
    }
}
