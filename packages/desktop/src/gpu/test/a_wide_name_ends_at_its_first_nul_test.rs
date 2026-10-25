use crate::gpu::wide_to_string;

#[test]
fn a_wide_name_ends_at_its_first_nul() {
    let name = [
        u16::from(b'\\'),
        u16::from(b'D'),
        u16::from(b'1'),
        0,
        u16::from(b'Z'),
    ];

    assert_eq!(wide_to_string(&name), "\\D1");
}
