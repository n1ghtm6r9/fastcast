use crate::avc::BitReader;

#[test]
fn reading_past_the_end_gives_none() {
    let mut reader = BitReader::new(&[0xff]);

    assert_eq!(reader.read_bits(8), Some(255));
    assert_eq!(reader.read_bit(), None);
    assert_eq!(reader.read_bits(1), None);
    assert_eq!(reader.read_ue(), None);
    assert_eq!(reader.read_se(), None);
}
