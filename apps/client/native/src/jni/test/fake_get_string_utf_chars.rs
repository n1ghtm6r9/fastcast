use crate::jni::test::{FakeObject, fake_object_behind};
use jni_sys::{JNIEnv, jboolean, jstring};
use std::ffi::c_char;
use std::ptr::null;

pub(crate) unsafe extern "system" fn fake_get_string_utf_chars(
    _env: *mut JNIEnv,
    string: jstring,
    is_copy: *mut jboolean,
) -> *const c_char {
    let FakeObject::Text(text) = (unsafe { fake_object_behind(string) }) else {
        return null();
    };

    if !is_copy.is_null() {
        unsafe { *is_copy = false };
    }

    text.as_ptr()
}
