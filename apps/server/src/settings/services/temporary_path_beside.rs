use std::path::{Path, PathBuf};

pub fn temporary_path_beside(path: &Path) -> PathBuf {
    let mut temporary = path.as_os_str().to_owned();
    temporary.push(".tmp");

    PathBuf::from(temporary)
}
