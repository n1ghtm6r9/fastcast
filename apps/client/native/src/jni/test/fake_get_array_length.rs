use crate::jni::test::{FakeObject, fake_object_behind};
use jni_sys::{JNIEnv, jarray, jsize};

pub(crate) unsafe extern "system" fn fake_get_array_length(
    _env: *mut JNIEnv,
    array: jarray,
) -> jsize {
    let length = match unsafe { fake_object_behind(array) } {
        FakeObject::Ints(ints) => ints.len(),
        FakeObject::Bytes(bytes) => bytes.borrow().len(),
        FakeObject::Text(_) => 0,
    };

    jsize::try_from(length).unwrap()
}
