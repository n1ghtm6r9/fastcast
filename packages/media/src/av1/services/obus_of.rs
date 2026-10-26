use crate::av1::{Obu, obu_at};

pub fn obus_of(data: &[u8]) -> Vec<Obu<'_>> {
    let mut obus = Vec::new();
    let mut rest = data;

    while let Some(obu) = obu_at(rest) {
        rest = &rest[obu.bytes.len()..];
        obus.push(obu);
    }

    obus
}
