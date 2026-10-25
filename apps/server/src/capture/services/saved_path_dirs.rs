use crate::capture::{PATH_REGISTRY_KEYS, saved_path_of};
use std::env;
use std::path::PathBuf;

pub fn saved_path_dirs() -> Vec<PathBuf> {
    PATH_REGISTRY_KEYS
        .iter()
        .filter_map(|&(root, key)| saved_path_of(root, key))
        .flat_map(|path| env::split_paths(&path).collect::<Vec<_>>())
        .collect()
}
