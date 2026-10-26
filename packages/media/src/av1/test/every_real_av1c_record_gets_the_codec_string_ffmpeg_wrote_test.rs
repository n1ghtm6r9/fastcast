use super::real_av1c_records;
use crate::av1_codec_string;

#[test]
fn every_real_av1c_record_gets_the_codec_string_ffmpeg_wrote() {
    for record in real_av1c_records() {
        assert_eq!(
            av1_codec_string(&record.av1c),
            Some(record.codec_string.to_string()),
            "{}",
            record.name
        );
    }
}
