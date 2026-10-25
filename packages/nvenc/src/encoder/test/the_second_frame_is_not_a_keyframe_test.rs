use super::{gradient_texture, nal_unit_types, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn the_second_frame_is_not_a_keyframe() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    let texture = gradient_texture(&device, 1920, 1080, 8, 0);
    encoder.encode(&texture, false).unwrap();

    let frame = encoder.encode(&texture, false).unwrap();

    let types = nal_unit_types(&frame.data, EncoderCodec::Hevc);
    assert!(!frame.keyframe);
    assert!(!types.contains(&19) && !types.contains(&20));
}
