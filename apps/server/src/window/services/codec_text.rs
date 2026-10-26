use media::VideoCodec;

pub fn codec_text(codec: VideoCodec, ten_bit: bool) -> String {
    let name = match codec {
        VideoCodec::Avc => "AVC",
        VideoCodec::Hevc => "HEVC",
    };

    if ten_bit {
        format!("{name} 10 бит")
    } else {
        name.to_owned()
    }
}
