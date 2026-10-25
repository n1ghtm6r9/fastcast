use super::real_hvcc_records;
use crate::hevc_codec_string;

#[test]
fn every_real_hvcc_record_gets_the_codec_string_the_spec_describes() {
    for record in real_hvcc_records() {
        assert_eq!(
            hevc_codec_string(&record.hvcc),
            Some(record.codec_string.to_string()),
            "{}",
            record.name
        );
    }
}
