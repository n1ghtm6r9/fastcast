use crate::packet::{ByteReader, CodecSupport, Hello, codec_from_id};

pub fn decode_hello(reader: &mut ByteReader<'_>) -> Option<Hello> {
    let nonce = reader.read_u32()?;
    let width = reader.read_u16()?;
    let height = reader.read_u16()?;
    let fps = reader.read_u16()?;
    let codec_count = reader.read_u8()?;
    let mut codecs = Vec::with_capacity(usize::from(codec_count));

    for _ in 0..codec_count {
        let codec_id = reader.read_u8()?;
        let max_bit_depth = reader.read_u8()?;

        if let Some(codec) = codec_from_id(codec_id) {
            codecs.push(CodecSupport {
                codec,
                max_bit_depth,
            });
        }
    }

    Some(Hello {
        nonce,
        width,
        height,
        fps,
        codecs,
    })
}
