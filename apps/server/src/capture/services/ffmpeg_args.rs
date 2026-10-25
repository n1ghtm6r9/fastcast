use crate::capture::{OUTPUT_ARGS, args_of, encoder_args, input_args, keyframe_interval_in_frames};
use crate::settings::{Settings, VideoEncoder};

pub fn ffmpeg_args(settings: &Settings, encoder: VideoEncoder) -> Vec<String> {
    let gop = keyframe_interval_in_frames(settings.capture_fps, settings.keyframe_interval_seconds);
    let mut args = input_args(settings.capture_output, settings.capture_fps);

    args.extend(encoder_args(encoder, settings.video_bitrate_kbps, gop));
    args.extend(args_of(OUTPUT_ARGS));

    args
}
