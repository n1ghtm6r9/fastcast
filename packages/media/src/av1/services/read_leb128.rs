use crate::av1::{LEB128_BITS_PER_BYTE, LEB128_MAX_BYTES, LEB128_MORE_FLAG, LEB128_VALUE_MASK};

pub fn read_leb128(data: &[u8]) -> Option<(u64, usize)> {
    let mut value = 0;
    let mut shift = 0;

    for (length, &byte) in (1..).zip(data.iter().take(LEB128_MAX_BYTES)) {
        value |= u64::from(byte & LEB128_VALUE_MASK) << shift;

        if byte & LEB128_MORE_FLAG == 0 {
            return Some((value, length));
        }

        shift += LEB128_BITS_PER_BYTE;
    }

    None
}
