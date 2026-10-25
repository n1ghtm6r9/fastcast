pub fn keyframe_interval_in_frames(fps: u32, seconds: u32) -> u32 {
    fps.saturating_mul(seconds)
}
