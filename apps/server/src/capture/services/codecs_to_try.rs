use crate::capture::codecs_or_avc;
use crate::settings::CodecChoice;
use media::VideoCodec;

pub fn codecs_to_try(choice: CodecChoice, offered: &[VideoCodec]) -> Vec<VideoCodec> {
    let mut codecs = codecs_or_avc(offered);
    let chosen_at = choice
        .codec()
        .and_then(|chosen| codecs.iter().position(|&codec| codec == chosen));
    if let Some(at) = chosen_at {
        codecs[..=at].rotate_right(1);
    }
    codecs
}
