use super::{RealAvccRecord, real_avcc_records};

pub fn real_avcc_record_named(name: &str) -> RealAvccRecord {
    real_avcc_records()
        .into_iter()
        .find(|record| record.name == name)
        .expect("the record exists")
}
