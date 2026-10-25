#[cfg(windows)]
mod encoder;
#[cfg(windows)]
mod ffi;

#[cfg(windows)]
pub use encoder::{EncodedFrame, Encoder, EncoderCodec, EncoderSettings, NvencError};
