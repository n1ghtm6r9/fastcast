use crate::hub::{Frame, MAX_GOP_FRAMES};
use std::sync::Arc;

pub fn record_in_gop(gop: &mut Vec<Arc<Frame>>, frame: &Arc<Frame>) {
    if frame.key {
        gop.clear();
        gop.push(frame.clone());
        return;
    }

    if gop.is_empty() {
        return;
    }

    if gop.len() >= MAX_GOP_FRAMES {
        gop.clear();
        return;
    }

    gop.push(frame.clone());
}
