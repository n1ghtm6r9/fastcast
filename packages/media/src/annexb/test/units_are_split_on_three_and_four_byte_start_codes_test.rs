use crate::annexb::annex_b_units;

#[test]
fn units_are_split_on_three_and_four_byte_start_codes() {
    let data = [
        0, 0, 0, 1, 0x67, 1, 2, 0, 0, 1, 0x68, 3, 0, 0, 0, 1, 0x65, 4, 5,
    ];

    assert_eq!(
        annex_b_units(&data),
        vec![&[0x67, 1, 2][..], &[0x68, 3][..], &[0x65, 4, 5][..]]
    );
}
