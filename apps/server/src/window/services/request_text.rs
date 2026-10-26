use crate::hub::ViewerRequest;
use crate::window::codec_text;

pub fn request_text(request: &ViewerRequest) -> String {
    let picture = match request.mode {
        Some(mode) => format!("{}×{} при {} Гц", mode.width, mode.height, mode.refresh_hz),
        None => "картинку по настройкам".to_owned(),
    };
    let codecs = request
        .codecs
        .iter()
        .map(|codec| codec_text(*codec, request.ten_bit_codecs.contains(codec)))
        .collect::<Vec<_>>();

    if codecs.is_empty() {
        format!("ТВ просит {picture}")
    } else {
        format!("ТВ просит {picture}, декодирует {}", codecs.join(", "))
    }
}
