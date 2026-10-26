use super::mode_of;
use crate::display::choose_mode;

#[test]
fn when_every_refresh_is_above_the_request_the_lowest_one_is_chosen() {
    let available = vec![
        mode_of(3840, 2160, 240),
        mode_of(3840, 2160, 60),
        mode_of(3840, 2160, 30),
        mode_of(1920, 1080, 10),
    ];

    assert_eq!(
        choose_mode(mode_of(3840, 2160, 25), &available),
        Some(mode_of(3840, 2160, 30))
    );
}
