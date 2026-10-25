use crate::fixtures::{config_of, nvenc_2560x1440_avcc};

#[test]
fn the_stream_config_exposes_its_codec_size_and_rate() {
    let avcc = nvenc_2560x1440_avcc();
    let config = config_of(&avcc, 60);

    assert_eq!(config.codec, "avc1.640033");
    assert_eq!((config.width, config.height, config.fps), (2560, 1440, 60));
    assert_eq!(config.record.to_vec(), avcc);
}
