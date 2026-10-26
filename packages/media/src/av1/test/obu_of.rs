use super::leb128_of;

pub fn obu_of(kind: u8, payload: &[u8]) -> Vec<u8> {
    let mut obu = vec![(kind << 3) | 0x02];

    obu.extend(leb128_of(payload.len()));
    obu.extend_from_slice(payload);

    obu
}
