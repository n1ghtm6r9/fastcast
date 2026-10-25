use crate::capture::keyframe_interval_in_frames;

#[test]
fn the_keyframe_interval_is_the_rate_times_the_seconds() {
    assert_eq!(keyframe_interval_in_frames(60, 2), 120);
    assert_eq!(keyframe_interval_in_frames(30, 1), 30);
    assert_eq!(keyframe_interval_in_frames(u32::MAX, 2), u32::MAX);
}
