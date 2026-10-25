use std::fs;
use std::path::{Path, PathBuf};
use std::{env, process};

pub struct TemporaryDirectory {
    path: PathBuf,
}

impl TemporaryDirectory {
    pub fn new(name: &str) -> Self {
        let path = env::temp_dir().join(format!("fastcast-test-{}-{name}", process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).expect("the temporary directory can be created");

        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn write(&self, relative_path: &str, bytes: &[u8]) -> PathBuf {
        let target = self.path.join(relative_path);
        let parent = target.parent().expect("a file has a parent directory");
        fs::create_dir_all(parent).expect("the parent directory can be created");
        fs::write(&target, bytes).expect("the file can be written");

        target
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
