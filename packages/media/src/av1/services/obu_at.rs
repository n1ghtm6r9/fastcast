use crate::av1::{
    OBU_EXTENSION_FLAG, OBU_EXTENSION_LENGTH, OBU_HAS_SIZE_FIELD, OBU_HEADER_LENGTH, OBU_TYPE_MASK,
    OBU_TYPE_SHIFT, Obu, read_leb128,
};

pub fn obu_at(data: &[u8]) -> Option<Obu<'_>> {
    let header = *data.first()?;
    let kind = (header >> OBU_TYPE_SHIFT) & OBU_TYPE_MASK;

    let header_length = if header & OBU_EXTENSION_FLAG == 0 {
        OBU_HEADER_LENGTH
    } else {
        OBU_HEADER_LENGTH + OBU_EXTENSION_LENGTH
    };

    let (payload_start, obu_length) = if header & OBU_HAS_SIZE_FIELD == 0 {
        (header_length, data.len())
    } else {
        let (size, size_length) = read_leb128(data.get(header_length..)?)?;
        let payload_start = header_length + size_length;

        (
            payload_start,
            payload_start.checked_add(usize::try_from(size).ok()?)?,
        )
    };

    let bytes = data.get(..obu_length)?;

    Some(Obu {
        kind,
        bytes,
        payload: bytes.get(payload_start..)?,
    })
}
