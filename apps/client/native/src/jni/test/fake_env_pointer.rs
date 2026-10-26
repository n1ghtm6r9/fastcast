use crate::jni::test::FakeEnv;
use jni_sys::JNIEnv;

impl FakeEnv {
    pub(crate) fn pointer(&mut self) -> *mut JNIEnv {
        &mut *self.interface
    }
}
