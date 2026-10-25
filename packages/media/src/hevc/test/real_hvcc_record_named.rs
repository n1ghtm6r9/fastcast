use super::{RealHvccRecord, real_hvcc_records};

pub fn real_hvcc_record_named(name: &str) -> RealHvccRecord {
    real_hvcc_records()
        .into_iter()
        .find(|record| record.name == name)
        .expect("the record exists")
}
