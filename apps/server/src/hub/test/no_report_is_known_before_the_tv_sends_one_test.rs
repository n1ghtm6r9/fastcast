use crate::hub::Hub;

#[test]
fn no_report_is_known_before_the_tv_sends_one() {
    let hub = Hub::new();

    assert_eq!(*hub.watch_reports().borrow(), None);
}
