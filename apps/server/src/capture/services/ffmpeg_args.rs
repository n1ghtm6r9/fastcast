use crate::capture::{
    CaptureTarget, KEYFRAME_INTERVAL_SECONDS, OUTPUT_ARGS, VideoEncoder, args_of, encoder_args,
    input_args, keyframe_interval_in_frames,
};
use crate::settings::Settings;
use media::VideoCodec;

pub fn ffmpeg_args(
    settings: &Settings,
    target: &CaptureTarget,
    encoder: VideoEncoder,
    codec: VideoCodec,
) -> Vec<String> {
    let gop = keyframe_interval_in_frames(target.fps, KEYFRAME_INTERVAL_SECONDS);
    let mut args = input_args(target.output, target.fps);

    args.extend(encoder_args(
        encoder,
        codec,
        settings.video_bitrate_kbps,
        gop,
    ));
    args.extend(args_of(OUTPUT_ARGS));

    args
}
