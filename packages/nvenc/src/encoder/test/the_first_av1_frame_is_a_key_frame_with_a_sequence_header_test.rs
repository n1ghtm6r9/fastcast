use super::{av1_frame_type, gradient_texture, nvidia_device, obu_types, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn the_first_av1_frame_is_a_key_frame_with_a_sequence_header() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Av1, 1920, 1080, 8)).unwrap();

    let frame = encoder
        .encode(&gradient_texture(&device, 1920, 1080, 8, 0), false)
        .unwrap();

    assert!(frame.keyframe);
    assert_eq!(obu_types(&frame.data)[..2], [2, 1]);
    assert_eq!(av1_frame_type(&frame.data), 0);
}
