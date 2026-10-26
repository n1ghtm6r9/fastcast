use std::fs::{self, File};
use std::io;
use std::path::Path;

pub fn open_journal_file(path: &Path) -> io::Result<File> {
    if let Some(directory) = path.parent() {
        fs::create_dir_all(directory)?;
    }

    File::create(path)
}
