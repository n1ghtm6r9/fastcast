use crate::stream::diagnose_notice_json;

#[test]
fn a_diagnose_notice_names_its_run() {
    assert_eq!(
        diagnose_notice_json(1_730_000_000_000),
        r#"{"type":"diagnose","run":1730000000000}"#
    );
}
