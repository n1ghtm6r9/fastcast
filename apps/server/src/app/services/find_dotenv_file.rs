use crate::app::DOTENV_FILE_NAME;
use std::path::{Path, PathBuf};

pub fn find_dotenv_file(directory: &Path) -> Option<PathBuf> {
    directory
        .ancestors()
        .map(|ancestor| ancestor.join(DOTENV_FILE_NAME))
        .find(|candidate| candidate.is_file())
}
