use crate::encoder::EncoderCodec;

pub fn nal_unit_types(annex_b: &[u8], codec: EncoderCodec) -> Vec<u8> {
    annex_b
        .windows(4)
        .filter(|window| window[..3] == [0, 0, 1])
        .map(|window| match codec {
            EncoderCodec::Avc => window[3] & 0x1F,
            EncoderCodec::Hevc => (window[3] >> 1) & 0x3F,
        })
        .collect()
}
