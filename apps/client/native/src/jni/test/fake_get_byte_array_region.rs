use crate::jni::test::{FakeObject, fake_object_behind};
use jni_sys::{JNIEnv, jbyte, jbyteArray, jsize};

pub(crate) unsafe extern "system" fn fake_get_byte_array_region(
    _env: *mut JNIEnv,
    array: jbyteArray,
    start: jsize,
    length: jsize,
    buffer: *mut jbyte,
) {
    let FakeObject::Bytes(bytes) = (unsafe { fake_object_behind(array) }) else {
        panic!("not a byte array");
    };
    let start = usize::try_from(start).unwrap();
    let length = usize::try_from(length).unwrap();

    for (offset, value) in bytes.borrow()[start..start + length].iter().enumerate() {
        unsafe { *buffer.add(offset) = *value };
    }
}
