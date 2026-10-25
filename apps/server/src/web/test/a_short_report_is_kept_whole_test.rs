use crate::web::clipped;

#[test]
fn a_short_report_is_kept_whole() {
    assert_eq!(clipped("{\"fps\":60}", 600), "{\"fps\":60}");
}
