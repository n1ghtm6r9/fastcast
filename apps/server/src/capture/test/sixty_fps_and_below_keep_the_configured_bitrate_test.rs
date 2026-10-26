use crate::capture::stream_bitrate_kbps;

#[test]
fn sixty_fps_and_below_keep_the_configured_bitrate() {
    assert_eq!(stream_bitrate_kbps(40_000, 60), 40_000);
    assert_eq!(stream_bitrate_kbps(40_000, 30), 40_000);
}
