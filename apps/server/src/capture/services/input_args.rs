use crate::capture::args_of;

pub fn input_args(output: u32, fps: u32) -> Vec<String> {
    args_of(&format!(
        "-hide_banner -loglevel warning -nostdin -f lavfi -i ddagrab=output_idx={output}:framerate={fps}:draw_mouse=1"
    ))
}
