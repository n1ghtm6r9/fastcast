use crate::display::DisplayScale;
use crate::stream::{SCALE_NOTICE_KIND, ScaleNotice};

pub fn scale_notice_json(scale: &DisplayScale) -> String {
    serde_json::to_string(&ScaleNotice {
        kind: SCALE_NOTICE_KIND,
        percent: scale.percent,
        min: scale.min,
        max: scale.max,
    })
    .expect("a notice made of numbers and a plain string always serializes")
}
