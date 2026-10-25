use super::FakeFfmpeg;

impl FakeFfmpeg {
    pub fn path(&self) -> String {
        self.directory
            .join("ffmpeg.cmd")
            .to_string_lossy()
            .into_owned()
    }
}
