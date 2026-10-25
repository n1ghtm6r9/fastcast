pub fn avc_codec_string(avcc: &[u8]) -> Option<String> {
    let [_, profile, compatibility, level, ..] = avcc else {
        return None;
    };

    Some(format!(
        "avc1.{:02x}{:02x}{:02x}",
        profile, compatibility, level
    ))
}
