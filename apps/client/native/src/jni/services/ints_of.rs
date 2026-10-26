use jni_sys::{JNIEnv, jintArray};

pub(crate) fn ints_of(env: *mut JNIEnv, array: jintArray) -> Vec<i32> {
    if array.is_null() {
        return Vec::new();
    }

    let functions = unsafe { &(**env).v1_1 };
    let length = unsafe { (functions.GetArrayLength)(env, array) }.max(0);
    let mut ints = vec![0; length as usize];

    unsafe { (functions.GetIntArrayRegion)(env, array, 0, length, ints.as_mut_ptr()) };

    ints
}
