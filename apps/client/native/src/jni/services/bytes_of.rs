use jni_sys::{JNIEnv, jbyteArray};

pub(crate) fn bytes_of(env: *mut JNIEnv, array: jbyteArray) -> Option<Vec<u8>> {
    if array.is_null() {
        return None;
    }

    let functions = unsafe { &(**env).v1_1 };
    let length = unsafe { (functions.GetArrayLength)(env, array) }.max(0);
    let mut bytes = vec![0u8; length as usize];

    unsafe { (functions.GetByteArrayRegion)(env, array, 0, length, bytes.as_mut_ptr().cast()) };

    Some(bytes)
}
