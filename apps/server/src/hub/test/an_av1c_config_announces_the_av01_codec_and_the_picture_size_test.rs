use crate::fixtures::nvenc_3840x2160_av1c;
use crate::hub::stream_config_from_record;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use bytes::Bytes;
use media::VideoCodec;

#[test]
fn an_av1c_config_announces_the_av01_codec_and_the_picture_size() {
    let av1c = nvenc_3840x2160_av1c();

    let config = stream_config_from_record(VideoCodec::Av1, Bytes::from(av1c.clone()), 120)
        .expect("the av1C describes a stream");

    assert_eq!(config.codec, "av01.0.14M.10");
    assert_eq!((config.width, config.height, config.fps), (3840, 2160, 120));
    assert_eq!(
        config.json,
        format!(
            r#"{{"type":"config","codec":"av01.0.14M.10","width":3840,"height":2160,"fps":120,"description":"{}"}}"#,
            STANDARD.encode(&av1c)
        )
    );
}
