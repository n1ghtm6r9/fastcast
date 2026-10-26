use crate::jni::test::FakeObject;
use jni_sys::jobject;

pub(crate) unsafe fn fake_object_behind<'a>(pointer: jobject) -> &'a FakeObject {
    unsafe { &*pointer.cast::<FakeObject>() }
}
