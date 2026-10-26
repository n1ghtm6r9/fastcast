use std::cell::RefCell;
use std::fs::File;

thread_local! {
    pub static HELD_GPU_SLOT: RefCell<Option<File>> = const { RefCell::new(None) };
}
