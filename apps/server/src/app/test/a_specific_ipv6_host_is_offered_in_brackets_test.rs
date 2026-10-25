use crate::app::urls_to_open;

#[test]
fn a_specific_ipv6_host_is_offered_in_brackets() {
    let urls = urls_to_open("fd00::1", 3278, None);

    assert_eq!(urls, vec!["http://[fd00::1]:3278"]);
}
