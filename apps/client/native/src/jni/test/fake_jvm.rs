use jni_sys::{JNIEnv, JNINativeInterface_};

pub(crate) struct FakeEnv {
    pub(crate) _table: Box<JNINativeInterface_>,
    pub(crate) interface: Box<JNIEnv>,
}
