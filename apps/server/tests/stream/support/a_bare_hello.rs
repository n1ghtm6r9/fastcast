use crate::support::next_nonce;
use wire::Hello;

pub fn a_bare_hello() -> Hello {
    Hello {
        nonce: next_nonce(),
        width: 0,
        height: 0,
        fps: 0,
        codecs: Vec::new(),
    }
}
