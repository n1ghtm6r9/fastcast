use crate::fixtures::{config_of, nvenc_2560x1440_avcc};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[test]
fn the_config_message_carries_codec_size_rate_and_the_base64_avcc() {
    let avcc = nvenc_2560x1440_avcc();
    let config = config_of(&avcc, 60);
    let expected = format!(
        r#"{{"type":"config","codec":"avc1.640033","width":2560,"height":1440,"fps":60,"description":"{}"}}"#,
        STANDARD.encode(&avcc)
    );

    assert_eq!(config.json, expected);
}
