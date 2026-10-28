use crate::link::socket_drops_in;

#[test]
fn a_full_memory_report_of_a_socket_gives_its_drops() {
    let report = [212_992, 2_097_152, 0, 212_992, 0, 0, 0, 0, 417];

    assert_eq!(socket_drops_in(&report, 36), Some(417));
}
