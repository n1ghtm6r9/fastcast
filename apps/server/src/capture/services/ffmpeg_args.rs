use crate::capture::{
    CaptureTarget, OUTPUT_ARGS, args_of, encoder_args, input_args, keyframe_interval_in_frames,
    stream_bitrate_kbps,
};
use crate::settings::{Settings, VideoEncoder};
use media::VideoCodec;

pub fn ffmpeg_args(
    settings: &Settings,
    target: &CaptureTarget,
    encoder: VideoEncoder,
    codec: VideoCodec,
) -> Vec<String> {
    let gop = keyframe_interval_in_frames(target.fps, settings.keyframe_interval_seconds);
    let mut args = input_args(target.output, target.fps);

    args.extend(encoder_args(
        encoder,
        codec,
        stream_bitrate_kbps(settings.video_bitrate_kbps, target.fps),
        gop,
    ));
    args.extend(args_of(OUTPUT_ARGS));

    args
}
