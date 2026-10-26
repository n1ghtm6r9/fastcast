use crate::capture::NativeError;
use nvenc::NvencError;
use std::error::Error;

#[test]
fn an_encoder_error_is_shown_as_it_is_and_kept_as_the_source() {
    let error = NativeError::Encoder(NvencError::DriverTooOld);

    assert_eq!(error.to_string(), NvencError::DriverTooOld.to_string());
    assert_eq!(
        error
            .source()
            .expect("the encoder error is the source")
            .to_string(),
        NvencError::DriverTooOld.to_string()
    );
}
