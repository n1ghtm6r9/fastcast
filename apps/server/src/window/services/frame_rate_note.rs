use crate::settings::FrameRate;

pub fn frame_rate_note(frame_rate: FrameRate, tv_refresh_hz: Option<u32>) -> Option<String> {
    let tv_refresh_hz = tv_refresh_hz?;
    frame_rate
        .limit()
        .is_none_or(|limit| limit > tv_refresh_hz)
        .then(|| format!("ТВ показывает не больше {tv_refresh_hz} кадров/с"))
}
