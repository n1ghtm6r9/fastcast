use media::VideoCodec;
use wire::{Codec, CodecSupport};

pub fn codecs_of(codecs: &[CodecSupport]) -> Vec<VideoCodec> {
    let mut chosen = Vec::new();

    for support in codecs {
        let codec = match support.codec {
            Codec::Avc => VideoCodec::Avc,
            Codec::Hevc => VideoCodec::Hevc,
            Codec::Av1 => VideoCodec::Av1,
        };

        if !chosen.contains(&codec) {
            chosen.push(codec);
        }
    }

    chosen
}
