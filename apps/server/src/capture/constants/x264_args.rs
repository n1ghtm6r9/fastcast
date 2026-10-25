pub const X264_ARGS: &str =
    "-vf hwdownload,format=bgra,format=yuv420p -c:v libx264 -preset ultrafast -tune zerolatency";
