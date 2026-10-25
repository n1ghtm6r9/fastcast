use super::settings;
use crate::encoder::{EncoderCodec, EncoderSettings, fc_settings_of};
use crate::ffi::fc_encoder_open;
use std::ffi::c_void;
use std::ptr::{dangling_mut, null_mut};

#[test]
fn the_c_layer_rejects_a_preset_outside_1_to_7() {
    let valid = settings(EncoderCodec::Hevc, 1920, 1080, 8);

    for preset in [0, 8] {
        let settings = fc_settings_of(&EncoderSettings { preset, ..valid });
        let mut encoder = null_mut();

        let result = unsafe { fc_encoder_open(dangling_mut::<c_void>(), &settings, &mut encoder) };

        assert_eq!((result.status, result.call), (-5, 0), "preset {}", preset);
        assert!(encoder.is_null());
    }
}
