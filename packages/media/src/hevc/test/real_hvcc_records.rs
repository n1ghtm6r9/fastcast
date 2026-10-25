use super::RealHvccRecord;
use crate::avc::test::hex_bytes;

const RECORDS: &str = include_str!("real_hvcc_records.txt");

pub fn real_hvcc_records() -> Vec<RealHvccRecord> {
    RECORDS
        .lines()
        .map(|line| {
            let mut columns = line.split_whitespace();
            let name = columns.next().expect("a record has a name");
            let width = columns.next().expect("a record has a width");
            let height = columns.next().expect("a record has a height");
            let codec_string = columns.next().expect("a record has a codec string");
            let hvcc = columns.next().expect("a record has the hvcC bytes");

            RealHvccRecord {
                name,
                width: width.parse().expect("the width is a number"),
                height: height.parse().expect("the height is a number"),
                codec_string,
                hvcc: hex_bytes(hvcc),
            }
        })
        .collect()
}
