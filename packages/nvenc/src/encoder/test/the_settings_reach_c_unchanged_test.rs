use super::settings;
use crate::encoder::{EncoderCodec, EncoderSettings, fc_settings_of};

#[test]
fn the_settings_reach_c_unchanged() {
    let hevc = fc_settings_of(&EncoderSettings {
        codec: EncoderCodec::Hevc,
        width: 3840,
        height: 2160,
        fps: 120,
        bitrate_kbps: 150_000,
        bit_depth: 10,
        preset: 7,
    });
    let avc = fc_settings_of(&settings(EncoderCodec::Avc, 1280, 720, 8));

    assert_eq!(
        (
            hevc.codec,
            hevc.width,
            hevc.height,
            hevc.fps,
            hevc.bitrate_kbps,
            hevc.bit_depth,
            hevc.preset
        ),
        (2, 3840, 2160, 120, 150_000, 10, 7)
    );
    assert_eq!(
        (
            avc.codec,
            avc.width,
            avc.height,
            avc.fps,
            avc.bitrate_kbps,
            avc.bit_depth,
            avc.preset
        ),
        (1, 1280, 720, 60, 50_000, 8, 4)
    );
}
