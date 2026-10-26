use super::{RealAv1cRecord, real_av1c_records};

pub fn real_av1c_record_named(name: &str) -> RealAv1cRecord {
    real_av1c_records()
        .into_iter()
        .find(|record| record.name == name)
        .expect("the record exists")
}
