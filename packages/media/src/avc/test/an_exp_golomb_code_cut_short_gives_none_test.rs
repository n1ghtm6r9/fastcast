use crate::avc::BitReader;

#[test]
fn an_exp_golomb_code_cut_short_gives_none() {
    assert_eq!(BitReader::new(&[0x00]).read_ue(), None);
    assert_eq!(BitReader::new(&[0x00, 0x01]).read_ue(), None);
    assert_eq!(BitReader::new(&[0x00, 0x01]).read_se(), None);
}
