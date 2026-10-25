use crate::display::poll_until;
use std::convert::Infallible;
use std::time::Duration;

#[test]
fn polling_returns_the_first_value_that_turns_up() {
    let mut attempts = 0;

    let found = poll_until(Duration::from_secs(5), Duration::from_millis(1), || {
        attempts += 1;

        Ok::<_, Infallible>((attempts == 3).then_some(attempts))
    });

    assert_eq!(found, Ok(Some(3)));
    assert_eq!(attempts, 3);
}
