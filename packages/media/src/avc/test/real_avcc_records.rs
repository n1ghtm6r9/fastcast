use super::{RealAvccRecord, hex_bytes};

const RECORDS: &str = include_str!("real_avcc_records.txt");

pub fn real_avcc_records() -> Vec<RealAvccRecord> {
    RECORDS
        .lines()
        .map(|line| {
            let mut columns = line.split_whitespace();
            let name = columns.next().expect("a record has a name");
            let width = columns.next().expect("a record has a width");
            let height = columns.next().expect("a record has a height");
            let avcc = columns.next().expect("a record has the avcC bytes");

            RealAvccRecord {
                name,
                width: width.parse().expect("the width is a number"),
                height: height.parse().expect("the height is a number"),
                avcc: hex_bytes(avcc),
            }
        })
        .collect()
}
