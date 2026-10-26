use crate::hub::TEN_BIT_DEPTH;
use crate::stream::codecs_of;
use media::VideoCodec;
use wire::CodecSupport;

pub fn ten_bit_codecs_of(codecs: &[CodecSupport]) -> Vec<VideoCodec> {
    let ten_bit: Vec<CodecSupport> = codecs
        .iter()
        .copied()
        .filter(|support| support.max_bit_depth >= TEN_BIT_DEPTH)
        .collect();

    codecs_of(&ten_bit)
}
