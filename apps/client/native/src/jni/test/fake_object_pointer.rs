use crate::jni::test::FakeObject;
use jni_sys::jobject;

impl FakeObject {
    pub(crate) fn pointer(&self) -> jobject {
        std::ptr::from_ref(self).cast_mut().cast()
    }
}
