use super::{gradient_texture, hevc_profile_idc, nvidia_device, settings};
use crate::encoder::{Encoder, EncoderCodec};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn eight_bit_hevc_is_main() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(&device, &settings(EncoderCodec::Hevc, 1920, 1080, 8)).unwrap();

    let frame = encoder
        .encode(&gradient_texture(&device, 1920, 1080, 8, 0), false)
        .unwrap();

    assert_eq!(hevc_profile_idc(&frame.data), 1);
}
