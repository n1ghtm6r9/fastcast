pub const FFMPEG_FILE_NAME: &str = if cfg!(windows) {
    "ffmpeg.exe"
} else {
    "ffmpeg"
};
