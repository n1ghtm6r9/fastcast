use crate::packet::Codec;

pub fn codec_from_id(id: u8) -> Option<Codec> {
    match id {
        1 => Some(Codec::Avc),
        2 => Some(Codec::Hevc),
        3 => Some(Codec::Av1),
        _ => None,
    }
}
