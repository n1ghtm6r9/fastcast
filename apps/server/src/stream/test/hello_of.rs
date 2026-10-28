use wire::{CodecSupport, Hello};

pub fn hello_of(width: u16, height: u16, fps: u16, codecs: Vec<CodecSupport>) -> Hello {
    Hello {
        nonce: 1,
        width,
        height,
        fps,
        codecs,
        noise: false,
    }
}
