use super::{av1_record_of, gradient_texture, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};
use media::av1_codec_string;

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn ten_bit_av1_is_main_profile_at_ten_bits() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Av1, 1920, 1080, 10)).unwrap();

    let frame = encoder
        .encode(&gradient_texture(&device, 1920, 1080, 10, 0), false)
        .unwrap();

    let codec_string = av1_codec_string(&av1_record_of(&frame.data)).unwrap();
    assert!(codec_string.starts_with("av01.0."), "{codec_string}");
    assert!(codec_string.ends_with(".10"), "{codec_string}");
}
