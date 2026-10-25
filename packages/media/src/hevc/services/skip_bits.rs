use crate::avc::BitReader;

pub fn skip_bits(reader: &mut BitReader, count: usize) -> Option<()> {
    for _ in 0..count {
        reader.read_bit()?;
    }

    Some(())
}
