use crate::app::data_directory_from;
use std::env;
use std::path::PathBuf;

pub fn data_directory() -> PathBuf {
    data_directory_from(|variable| env::var_os(variable))
}
