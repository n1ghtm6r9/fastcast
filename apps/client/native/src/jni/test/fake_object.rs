use std::cell::RefCell;
use std::ffi::CString;

#[derive(Debug)]
pub(crate) enum FakeObject {
    Text(CString),
    Ints(Vec<i32>),
    Bytes(RefCell<Vec<i8>>),
}
