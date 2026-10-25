use super::{gradient_texture, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec, NvencError};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_texture_of_another_size_is_rejected() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();

    let result = encoder.encode(&gradient_texture(&device, 1280, 720, 8, 0), false);

    assert_eq!(result, Err(NvencError::TextureMismatch));
}
