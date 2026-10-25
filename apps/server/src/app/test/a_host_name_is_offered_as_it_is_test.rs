use crate::app::urls_to_open;

#[test]
fn a_host_name_is_offered_as_it_is() {
    let urls = urls_to_open("tv.local", 3278, None);

    assert_eq!(urls, vec!["http://tv.local:3278"]);
}
