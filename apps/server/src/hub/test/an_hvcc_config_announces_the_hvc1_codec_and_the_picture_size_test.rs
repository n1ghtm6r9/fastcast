use crate::fixtures::nvenc_3840x2160_hvcc;
use crate::hub::stream_config_from_record;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bytes::Bytes;
use media::VideoCodec;

#[test]
fn an_hvcc_config_announces_the_hvc1_codec_and_the_picture_size() {
    let hvcc = nvenc_3840x2160_hvcc();

    let config = stream_config_from_record(VideoCodec::Hevc, Bytes::from(hvcc.clone()), 60)
        .expect("the hvcC describes a stream");

    assert_eq!(config.codec, "hvc1.1.6.L153.90");
    assert_eq!((config.width, config.height, config.fps), (3840, 2160, 60));
    assert_eq!(
        config.json,
        format!(
            r#"{{"type":"config","codec":"hvc1.1.6.L153.90","width":3840,"height":2160,"fps":60,"description":"{}"}}"#,
            STANDARD.encode(&hvcc)
        )
    );
}
