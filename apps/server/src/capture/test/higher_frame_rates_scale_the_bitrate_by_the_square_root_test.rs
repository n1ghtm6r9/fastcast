use crate::capture::stream_bitrate_kbps;

#[test]
fn higher_frame_rates_scale_the_bitrate_by_the_square_root() {
    assert_eq!(stream_bitrate_kbps(40_000, 120), 56_569);
    assert_eq!(stream_bitrate_kbps(200_000, 240), 400_000);
}
