use super::nal_units;
use crate::encoder::EncoderCodec;

pub fn nal_unit_types(annex_b: &[u8], codec: EncoderCodec) -> Vec<u8> {
    nal_units(annex_b)
        .iter()
        .map(|unit| match codec {
            EncoderCodec::Avc => unit[0] & 0x1F,
            EncoderCodec::Hevc => (unit[0] >> 1) & 0x3F,
            EncoderCodec::Av1 => panic!("AV1 has OBUs, not NAL units"),
        })
        .collect()
}
