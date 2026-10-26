use crate::jni::test::{FakeObject, fake_object_behind};
use jni_sys::{JNIEnv, jbyte, jbyteArray, jsize};
use std::slice::from_raw_parts;

pub(crate) unsafe extern "system" fn fake_set_byte_array_region(
    _env: *mut JNIEnv,
    array: jbyteArray,
    start: jsize,
    length: jsize,
    buffer: *const jbyte,
) {
    let FakeObject::Bytes(bytes) = (unsafe { fake_object_behind(array) }) else {
        panic!("not a byte array");
    };
    let start = usize::try_from(start).unwrap();
    let length = usize::try_from(length).unwrap();
    let source = unsafe { from_raw_parts(buffer, length) };

    bytes.borrow_mut()[start..start + length].copy_from_slice(source);
}
