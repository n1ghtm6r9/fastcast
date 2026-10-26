use crate::jni::test::FakeObject;

pub(crate) fn ints(values: &[i32]) -> FakeObject {
    FakeObject::Ints(values.to_vec())
}
