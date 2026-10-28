use crate::link::hello_of;
use crate::link::test::TEST_NONCE;
use wire::Hello;

pub(crate) fn test_hello() -> Hello {
    hello_of(TEST_NONCE, 3840, 2160, 120, &[2, 10, 1, 8], false)
}
