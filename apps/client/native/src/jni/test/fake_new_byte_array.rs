use crate::jni::test::FakeObject;
use jni_sys::{JNIEnv, jbyteArray, jsize};
use std::cell::RefCell;

pub(crate) unsafe extern "system" fn fake_new_byte_array(
    _env: *mut JNIEnv,
    length: jsize,
) -> jbyteArray {
    let length = usize::try_from(length).unwrap();
    let array = Box::new(FakeObject::Bytes(RefCell::new(vec![0; length])));

    Box::into_raw(array).cast()
}
