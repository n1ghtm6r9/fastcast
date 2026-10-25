use crate::packet::Hello;

pub fn write_hello(hello: &Hello, out: &mut Vec<u8>) {
    let count = u8::try_from(hello.codecs.len()).unwrap_or(u8::MAX);

    out.extend_from_slice(&hello.nonce.to_le_bytes());
    out.extend_from_slice(&hello.width.to_le_bytes());
    out.extend_from_slice(&hello.height.to_le_bytes());
    out.extend_from_slice(&hello.fps.to_le_bytes());
    out.push(count);

    for support in hello.codecs.iter().take(usize::from(count)) {
        out.push(support.codec as u8);
        out.push(support.max_bit_depth);
    }
}
