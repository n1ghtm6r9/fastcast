use crate::app::urls_to_open;

#[test]
fn a_specific_ipv4_host_is_offered_as_it_is() {
    let urls = urls_to_open("192.168.0.220", 3278, None);

    assert_eq!(urls, vec!["http://192.168.0.220:3278"]);
}
