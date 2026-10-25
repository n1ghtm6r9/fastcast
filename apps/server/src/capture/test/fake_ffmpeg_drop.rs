use super::FakeFfmpeg;
use std::fs;

impl Drop for FakeFfmpeg {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}
