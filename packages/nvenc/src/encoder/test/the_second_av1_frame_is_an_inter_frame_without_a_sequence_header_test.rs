use super::{av1_frame_type, gradient_texture, nvidia_device, obu_types, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn the_second_av1_frame_is_an_inter_frame_without_a_sequence_header() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Av1, 1920, 1080, 8)).unwrap();
    let texture = gradient_texture(&device, 1920, 1080, 8, 0);
    encoder.encode(&texture, false).unwrap();

    let frame = encoder.encode(&texture, false).unwrap();

    assert!(!frame.keyframe);
    assert!(!obu_types(&frame.data).contains(&1));
    assert_eq!(av1_frame_type(&frame.data), 1);
}
