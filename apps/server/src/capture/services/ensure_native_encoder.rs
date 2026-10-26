use crate::capture::CaptureError;
use crate::settings::{EncoderChoice, VideoEncoder};

pub fn ensure_native_encoder(choice: EncoderChoice) -> Result<(), CaptureError> {
    match choice {
        EncoderChoice::Fixed(encoder) if encoder != VideoEncoder::Nvenc => {
            Err(CaptureError::EncoderNotNative { encoder })
        }
        _ => Ok(()),
    }
}
