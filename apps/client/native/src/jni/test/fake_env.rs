use crate::jni::test::{
    FakeEnv, fake_get_array_length, fake_get_byte_array_region, fake_get_int_array_region,
    fake_get_string_utf_chars, fake_new_byte_array, fake_release_string_utf_chars,
    fake_set_byte_array_region,
};
use jni_sys::JNINativeInterface_;

pub(crate) fn fake_env() -> FakeEnv {
    let mut table: Box<JNINativeInterface_> = Box::new(unsafe { std::mem::zeroed() });

    table.v1_1.GetStringUTFChars = fake_get_string_utf_chars;
    table.v1_1.ReleaseStringUTFChars = fake_release_string_utf_chars;
    table.v1_1.GetArrayLength = fake_get_array_length;
    table.v1_1.GetIntArrayRegion = fake_get_int_array_region;
    table.v1_1.GetByteArrayRegion = fake_get_byte_array_region;
    table.v1_1.NewByteArray = fake_new_byte_array;
    table.v1_1.SetByteArrayRegion = fake_set_byte_array_region;

    let interface = Box::new(std::ptr::from_ref(&*table));

    FakeEnv {
        _table: table,
        interface,
    }
}
