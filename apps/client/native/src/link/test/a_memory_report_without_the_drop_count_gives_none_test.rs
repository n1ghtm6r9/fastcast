use crate::link::socket_drops_in;

#[test]
fn a_memory_report_without_the_drop_count_gives_none() {
    let report = [212_992, 2_097_152, 0, 212_992, 0, 0, 0, 0, 417];

    assert_eq!(socket_drops_in(&report, 32), None);
}
