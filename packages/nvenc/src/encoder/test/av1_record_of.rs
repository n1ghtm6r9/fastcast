use media::{VideoCodec, split_access_unit};

pub fn av1_record_of(temporal_unit: &[u8]) -> Vec<u8> {
    split_access_unit(VideoCodec::Av1, temporal_unit)
        .record
        .expect("a sequence header media can read")
}
