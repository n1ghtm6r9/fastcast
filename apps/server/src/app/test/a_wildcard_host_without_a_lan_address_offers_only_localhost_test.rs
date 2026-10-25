use crate::app::urls_to_open;

#[test]
fn a_wildcard_host_without_a_lan_address_offers_only_localhost() {
    let urls = urls_to_open("0.0.0.0", 3278, None);

    assert_eq!(urls, vec!["http://localhost:3278"]);
}
