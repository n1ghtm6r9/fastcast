use crate::support::{ARRIVAL_LIMIT, TestViewer};
use serde_json::Value;
use wire::FrameKind;

pub fn next_config(viewer: &mut TestViewer) -> Value {
    let frame = viewer
        .next_frame(ARRIVAL_LIMIT)
        .expect("a config frame arrives in time");

    assert_eq!(frame.kind, FrameKind::Config, "expected the config");

    serde_json::from_slice(&frame.payload).expect("the config is JSON")
}
