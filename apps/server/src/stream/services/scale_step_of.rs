use crate::stream::{SCALE_NOTICE_KIND, ScaleAsk};

pub fn scale_step_of(notice: &str) -> Option<i32> {
    serde_json::from_str::<ScaleAsk>(notice)
        .ok()
        .filter(|ask| ask.kind == SCALE_NOTICE_KIND)
        .map(|ask| ask.step)
}
