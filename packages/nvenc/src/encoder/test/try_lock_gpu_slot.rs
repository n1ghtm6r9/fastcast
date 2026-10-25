use std::env::temp_dir;
use std::fs::{File, OpenOptions, TryLockError};

pub fn try_lock_gpu_slot(slot: usize) -> Option<File> {
    let path = temp_dir().join(format!("fastcast-nvenc-test-slot-{slot}.lock"));
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(&path)
        .unwrap();

    match file.try_lock() {
        Ok(()) => Some(file),
        Err(TryLockError::WouldBlock) => None,
        Err(TryLockError::Error(error)) => panic!("cannot lock {}: {}", path.display(), error),
    }
}
