use super::mode_of;
use crate::display::choose_mode;

#[test]
fn a_refresh_the_display_lacks_is_shown_at_its_lowest_offered_multiple() {
    let available = vec![
        mode_of(3840, 2160, 24),
        mode_of(3840, 2160, 30),
        mode_of(3840, 2160, 60),
        mode_of(3840, 2160, 144),
        mode_of(3840, 2160, 240),
    ];

    assert_eq!(
        choose_mode(mode_of(3840, 2160, 120), &available),
        Some(mode_of(3840, 2160, 240))
    );
}
