use super::mode_of;
use crate::display::choose_mode;

#[test]
fn a_mode_without_a_refresh_rate_is_no_multiple_of_the_request() {
    let available = vec![mode_of(3840, 2160, 0), mode_of(3840, 2160, 60)];

    assert_eq!(
        choose_mode(mode_of(3840, 2160, 60), &available),
        Some(mode_of(3840, 2160, 60))
    );
}
