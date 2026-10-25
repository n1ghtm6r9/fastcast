use crate::fec::recover_data;

#[test]
fn a_present_list_of_the_wrong_length_cannot_be_recovered() {
    let mut block = vec![7u8; 3 * 8 + 2 * 8];
    let present = [false, true, true, true];

    assert_eq!(recover_data(3, 2, 8, &mut block, &present), None);
    assert!(block.iter().all(|&byte| byte == 7));
}
