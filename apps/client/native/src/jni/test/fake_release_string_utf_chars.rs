use jni_sys::{JNIEnv, jstring};
use std::ffi::c_char;

pub(crate) unsafe extern "system" fn fake_release_string_utf_chars(
    _env: *mut JNIEnv,
    _string: jstring,
    _chars: *const c_char,
) {
}
