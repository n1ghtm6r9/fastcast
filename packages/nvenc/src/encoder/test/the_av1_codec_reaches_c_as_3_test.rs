use super::settings;
use crate::encoder::{EncoderCodec, fc_settings_of};

#[test]
fn the_av1_codec_reaches_c_as_3() {
    assert_eq!(
        fc_settings_of(&settings(EncoderCodec::Av1, 1920, 1080, 8)).codec,
        3
    );
}
