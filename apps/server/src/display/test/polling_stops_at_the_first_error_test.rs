use crate::display::poll_until;
use std::time::Duration;

#[test]
fn polling_stops_at_the_first_error() {
    let mut attempts = 0;

    let result = poll_until(Duration::from_secs(5), Duration::from_millis(1), || {
        attempts += 1;

        Err::<Option<u32>, _>("the driver went away")
    });

    assert_eq!(result, Err("the driver went away"));
    assert_eq!(attempts, 1);
}
