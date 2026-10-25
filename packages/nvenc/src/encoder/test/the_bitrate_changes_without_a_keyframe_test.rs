use super::{gradient_texture, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn the_bitrate_changes_without_a_keyframe() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    let texture = gradient_texture(&device, 1920, 1080, 8, 0);
    encoder.encode(&texture, false).unwrap();
    encoder.encode(&texture, false).unwrap();

    assert_eq!(encoder.set_bitrate(25_000), Ok(()));
    let frame = encoder.encode(&texture, false).unwrap();

    assert!(!frame.keyframe);
}
