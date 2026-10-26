use crate::jni::test::{FakeObject, fake_object_behind};
use jni_sys::jbyteArray;

pub(crate) fn bytes_back(array: jbyteArray) -> Vec<u8> {
    let FakeObject::Bytes(bytes) = (unsafe { fake_object_behind(array) }) else {
        panic!("not a byte array");
    };

    bytes.borrow().iter().map(|value| *value as u8).collect()
}
