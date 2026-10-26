use super::settings;
use crate::encoder::{EncoderCodec, EncoderSettings, fc_settings_of};
use crate::ffi::{FcSettings, fc_encoder_open};
use std::ffi::c_void;
use std::ptr::{dangling_mut, null_mut};

fn rejected_status(settings: &FcSettings) -> (i32, u32) {
    let mut encoder = null_mut();
    let result = unsafe { fc_encoder_open(dangling_mut::<c_void>(), settings, &mut encoder) };
    assert!(encoder.is_null());
    (result.status, result.call)
}

#[test]
fn the_c_layer_rejects_every_setting_outside_its_range() {
    let valid = settings(EncoderCodec::Hevc, 1920, 1080, 8);
    let variants = [
        EncoderSettings { width: 0, ..valid },
        EncoderSettings { width: 1, ..valid },
        EncoderSettings {
            width: 8194,
            ..valid
        },
        EncoderSettings { height: 0, ..valid },
        EncoderSettings {
            height: 1079,
            ..valid
        },
        EncoderSettings {
            height: 8194,
            ..valid
        },
        EncoderSettings { fps: 0, ..valid },
        EncoderSettings { fps: 241, ..valid },
        EncoderSettings {
            bitrate_kbps: 0,
            ..valid
        },
        EncoderSettings {
            bitrate_kbps: 800_001,
            ..valid
        },
        EncoderSettings {
            bit_depth: 0,
            ..valid
        },
        EncoderSettings {
            bit_depth: 9,
            ..valid
        },
        EncoderSettings {
            bit_depth: 12,
            ..valid
        },
    ];

    for variant in variants {
        assert_eq!(
            rejected_status(&fc_settings_of(&variant)),
            (-5, 0),
            "{:?}",
            variant
        );
    }

    for codec in [0, 4] {
        let settings = FcSettings {
            codec,
            ..fc_settings_of(&valid)
        };
        assert_eq!(rejected_status(&settings), (-5, 0), "codec {}", codec);
    }
}
