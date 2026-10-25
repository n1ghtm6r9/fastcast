use crate::capture::probe_args;
use crate::settings::VideoEncoder;

#[test]
fn the_probe_encodes_one_black_frame_with_the_chosen_codec() {
    assert_eq!(
        probe_args(VideoEncoder::Nvenc).join(" "),
        "-hide_banner -loglevel error -f lavfi -i color=black:size=640x360:rate=30 -frames:v 1 -c:v h264_nvenc -f null -"
    );
}
