use jni_sys::{JNIEnv, jstring};
use std::ffi::CStr;
use std::ptr::null_mut;

pub(crate) fn string_of(env: *mut JNIEnv, string: jstring) -> Option<String> {
    if string.is_null() {
        return None;
    }

    let functions = unsafe { &(**env).v1_1 };
    let chars = unsafe { (functions.GetStringUTFChars)(env, string, null_mut()) };

    if chars.is_null() {
        return None;
    }

    let text = unsafe { CStr::from_ptr(chars) }
        .to_str()
        .ok()
        .map(str::to_owned);

    unsafe { (functions.ReleaseStringUTFChars)(env, string, chars) };

    text
}
