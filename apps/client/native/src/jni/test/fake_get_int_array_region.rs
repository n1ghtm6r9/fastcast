use crate::jni::test::{FakeObject, fake_object_behind};
use jni_sys::{JNIEnv, jint, jintArray, jsize};

pub(crate) unsafe extern "system" fn fake_get_int_array_region(
    _env: *mut JNIEnv,
    array: jintArray,
    start: jsize,
    length: jsize,
    buffer: *mut jint,
) {
    let FakeObject::Ints(ints) = (unsafe { fake_object_behind(array) }) else {
        panic!("not an int array");
    };
    let start = usize::try_from(start).unwrap();
    let length = usize::try_from(length).unwrap();

    for (offset, value) in ints[start..start + length].iter().enumerate() {
        unsafe { *buffer.add(offset) = *value };
    }
}
