use super::{gradient_texture, nal_unit_types, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_forced_keyframe_is_an_idr_with_parameter_sets() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();
    let texture = gradient_texture(&device, 1920, 1080, 8, 0);
    for _ in 0..3 {
        encoder.encode(&texture, false).unwrap();
    }

    let frame = encoder.encode(&texture, true).unwrap();

    let types = nal_unit_types(&frame.data, EncoderCodec::Hevc);
    assert!(frame.keyframe);
    assert!([32, 33, 34].iter().all(|kind| types.contains(kind)));
    assert!(types.contains(&19) || types.contains(&20));
}
