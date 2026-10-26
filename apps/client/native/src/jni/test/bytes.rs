use crate::jni::test::FakeObject;
use std::cell::RefCell;

pub(crate) fn bytes(values: &[u8]) -> FakeObject {
    FakeObject::Bytes(RefCell::new(
        values.iter().map(|value| *value as i8).collect(),
    ))
}
