use crate::hub::Hub;

#[test]
fn reset_keeps_the_diagnose_run() {
    let hub = Hub::new();

    hub.diagnose(42);
    hub.reset();

    assert_eq!(hub.join().diagnose, Some(42));
}
