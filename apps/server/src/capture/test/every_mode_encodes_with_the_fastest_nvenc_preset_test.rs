use crate::capture::{NativePipeline, encoder_settings_for};
use crate::hub::TEN_BIT_DEPTH;
use media::VideoCodec;
use nvenc::{EncoderCodec, EncoderSettings};

#[test]
fn every_mode_encodes_with_the_fastest_nvenc_preset() {
    let four_k = NativePipeline {
        codec: VideoCodec::Hevc,
        bit_depth: TEN_BIT_DEPTH,
        noise: false,
    };
    let quad_hd = NativePipeline {
        codec: VideoCodec::Avc,
        bit_depth: 8,
        noise: false,
    };

    assert_eq!(
        encoder_settings_for(four_k, (3840, 2160), 60, 150_000),
        EncoderSettings {
            codec: EncoderCodec::Hevc,
            width: 3840,
            height: 2160,
            fps: 60,
            bitrate_kbps: 150_000,
            bit_depth: TEN_BIT_DEPTH,
            preset: 1,
        }
    );
    assert_eq!(
        encoder_settings_for(quad_hd, (2560, 1440), 120, 80_000),
        EncoderSettings {
            codec: EncoderCodec::Avc,
            width: 2560,
            height: 1440,
            fps: 120,
            bitrate_kbps: 80_000,
            bit_depth: 8,
            preset: 1,
        }
    );
}
