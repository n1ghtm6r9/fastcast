use super::{av1_record_of, gradient_texture, nvidia_device};
use crate::encoder::{Encoder, EncoderCodec, EncoderSettings};
use media::{VideoSize, av1_codec_string, video_size_from_av1c};

#[test]
#[ignore = "needs an NVIDIA GPU"]
fn a_4k_120_ten_bit_av1_stream_is_level_5_2_main_tier() {
    let device = nvidia_device();
    let mut encoder = Encoder::open(
        &device,
        &EncoderSettings {
            codec: EncoderCodec::Av1,
            width: 3840,
            height: 2160,
            fps: 120,
            bitrate_kbps: 40_000,
            bit_depth: 10,
            preset: 1,
        },
    )
    .unwrap();

    let frame = encoder
        .encode(&gradient_texture(&device, 3840, 2160, 10, 0), false)
        .unwrap();

    let record = av1_record_of(&frame.data);
    assert_eq!(av1_codec_string(&record).as_deref(), Some("av01.0.14M.10"));
    assert_eq!(
        video_size_from_av1c(&record),
        Some(VideoSize {
            width: 3840,
            height: 2160
        })
    );
}
