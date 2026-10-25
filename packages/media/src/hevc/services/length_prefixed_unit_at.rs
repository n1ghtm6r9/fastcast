use crate::hevc::{HVCC_NAL_UNIT_LENGTH_FIELD_SIZE, u16_at};

pub fn length_prefixed_unit_at(bytes: &[u8], position: usize) -> Option<&[u8]> {
    let length = usize::from(u16_at(bytes, position)?);
    let start = position.checked_add(HVCC_NAL_UNIT_LENGTH_FIELD_SIZE)?;

    bytes.get(start..start.checked_add(length)?)
}
