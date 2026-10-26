use super::{gradient_texture, hevc_level_idc, hevc_tier_flag, nvidia_device};
use crate::encoder::{Encoder, EncoderCodec, EncoderSettings};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_4k_120_ten_bit_stream_is_level_5_2_main_tier() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(
        &device,
        &EncoderSettings {
            codec: EncoderCodec::Hevc,
            width: 3840,
            height: 2160,
            fps: 120,
            bitrate_kbps: 56_569,
            bit_depth: 10,
            preset: 1,
        },
    )
    .unwrap();

    let frame = encoder
        .encode(&gradient_texture(&device, 3840, 2160, 10, 0), false)
        .unwrap();

    assert_eq!(hevc_level_idc(&frame.data), 156);
    assert_eq!(hevc_tier_flag(&frame.data), 0);
}
