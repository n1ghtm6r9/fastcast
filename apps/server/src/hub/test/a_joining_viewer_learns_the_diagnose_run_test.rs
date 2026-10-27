use crate::hub::Hub;

#[test]
fn a_joining_viewer_learns_the_diagnose_run() {
    let hub = Hub::new();

    hub.diagnose(1_730_000_000_000);

    assert_eq!(hub.join().diagnose, Some(1_730_000_000_000));
}
