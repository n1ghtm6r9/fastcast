use crate::jni::test::FakeObject;
use std::ffi::CString;

pub(crate) fn text(value: &str) -> FakeObject {
    FakeObject::Text(CString::new(value).unwrap())
}
