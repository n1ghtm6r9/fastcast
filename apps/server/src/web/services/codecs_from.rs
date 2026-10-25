use crate::web::codec_named;
use media::VideoCodec;

pub fn codecs_from(list: Option<&str>) -> Vec<VideoCodec> {
    let mut codecs = Vec::new();

    for codec in list.unwrap_or_default().split(',').filter_map(codec_named) {
        if !codecs.contains(&codec) {
            codecs.push(codec);
        }
    }

    codecs
}
