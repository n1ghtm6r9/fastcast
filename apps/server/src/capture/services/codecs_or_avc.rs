use media::VideoCodec;

pub fn codecs_or_avc(codecs: &[VideoCodec]) -> Vec<VideoCodec> {
    if codecs.is_empty() {
        return vec![VideoCodec::Avc];
    }

    codecs.to_vec()
}
