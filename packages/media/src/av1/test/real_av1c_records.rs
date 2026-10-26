use super::RealAv1cRecord;
use crate::avc::test::hex_bytes;

const RECORDS: &str = include_str!("real_av1c_records.txt");

pub fn real_av1c_records() -> Vec<RealAv1cRecord> {
    RECORDS
        .lines()
        .map(|line| {
            let mut columns = line.split_whitespace();
            let name = columns.next().expect("a record has a name");
            let width = columns.next().expect("a record has a width");
            let height = columns.next().expect("a record has a height");
            let codec_string = columns.next().expect("a record has a codec string");
            let av1c = columns.next().expect("a record has the av1C bytes");

            RealAv1cRecord {
                name,
                width: width.parse().expect("the width is a number"),
                height: height.parse().expect("the height is a number"),
                codec_string,
                av1c: hex_bytes(av1c),
            }
        })
        .collect()
}
