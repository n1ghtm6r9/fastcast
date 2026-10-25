use crate::capture::args_of;

pub fn rate_control_args(bitrate_kbps: u32, gop: u32) -> Vec<String> {
    args_of(&format!(
        "-b:v {bitrate_kbps}k -maxrate {bitrate_kbps}k -bufsize {bitrate_kbps}k -g {gop} -bf 0 -profile:v high"
    ))
}
