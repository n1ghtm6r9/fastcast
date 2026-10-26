use crate::link::LinkEvent;
use jni_sys::{JNIEnv, jbyte, jbyteArray, jsize};
use std::ptr::null_mut;

pub(crate) fn event_array(env: *mut JNIEnv, event: &LinkEvent) -> jbyteArray {
    let payload = event.payload();
    let Ok(length) = jsize::try_from(payload.len() + 1) else {
        return null_mut();
    };

    let functions = unsafe { &(**env).v1_1 };
    let array = unsafe { (functions.NewByteArray)(env, length) };

    if array.is_null() {
        return null_mut();
    }

    let kind = event.kind() as jbyte;

    unsafe { (functions.SetByteArrayRegion)(env, array, 0, 1, &kind) };

    if !payload.is_empty() {
        unsafe {
            (functions.SetByteArrayRegion)(env, array, 1, length - 1, payload.as_ptr().cast())
        };
    }

    array
}
