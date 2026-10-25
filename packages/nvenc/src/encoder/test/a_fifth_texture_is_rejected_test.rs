use super::{gradient_texture, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec, NvencError};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_fifth_texture_is_rejected() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    let textures: Vec<_> = (0..5)
        .map(|shift| gradient_texture(&device, 1920, 1080, 8, shift))
        .collect();
    for texture in &textures[..4] {
        encoder.encode(texture, false).unwrap();
    }

    let fifth = encoder.encode(&textures[4], false);
    let first_again = encoder.encode(&textures[0], false);

    assert_eq!(fifth, Err(NvencError::TextureLimit));
    assert!(first_again.is_ok());
}
