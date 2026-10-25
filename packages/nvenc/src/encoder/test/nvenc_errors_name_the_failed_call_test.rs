use crate::encoder::NvencError;

#[test]
fn nvenc_errors_name_the_failed_call() {
    let error = NvencError::Status {
        call: "nvEncEncodePicture",
        status: 8,
    };

    assert_eq!(
        error.to_string(),
        "nvEncEncodePicture failed with NVENCSTATUS 8"
    );
}
