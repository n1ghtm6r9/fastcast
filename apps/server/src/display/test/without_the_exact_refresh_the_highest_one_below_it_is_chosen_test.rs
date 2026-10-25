use super::mode_of;
use crate::display::choose_mode;

#[test]
fn without_the_exact_refresh_the_highest_one_below_it_is_chosen() {
    let available = vec![
        mode_of(1920, 1080, 100),
        mode_of(3840, 2160, 240),
        mode_of(3840, 2160, 144),
        mode_of(3840, 2160, 60),
        mode_of(3840, 2160, 30),
        mode_of(3840, 2160, 24),
    ];

    assert_eq!(
        choose_mode(mode_of(3840, 2160, 100), &available),
        Some(mode_of(3840, 2160, 60))
    );
    assert_eq!(
        choose_mode(mode_of(3840, 2160, 200), &available),
        Some(mode_of(3840, 2160, 144))
    );
}
