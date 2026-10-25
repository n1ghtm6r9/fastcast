use crate::capture::CaptureError;
use std::io::{self, ErrorKind};

pub fn spawn_error_of(path: &str, source: io::Error) -> CaptureError {
    if source.kind() == ErrorKind::NotFound {
        CaptureError::FfmpegNotFound {
            path: path.to_owned(),
        }
    } else {
        CaptureError::Spawn {
            path: path.to_owned(),
            source,
        }
    }
}
