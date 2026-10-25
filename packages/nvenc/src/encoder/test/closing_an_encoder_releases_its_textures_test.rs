use super::{gradient_texture, nvidia_device, reference_count, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn closing_an_encoder_releases_its_textures() {
    let device = nvidia_device();
    let texture = gradient_texture(&device, 1920, 1080, 8, 0);
    let before = reference_count(&texture);
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    encoder.encode(&texture, false).unwrap();

    drop(encoder);

    assert_eq!(reference_count(&texture), before);
}
