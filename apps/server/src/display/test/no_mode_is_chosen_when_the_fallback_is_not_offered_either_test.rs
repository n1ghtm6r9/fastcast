use super::mode_of;
use crate::display::choose_mode;

#[test]
fn no_mode_is_chosen_when_the_fallback_is_not_offered_either() {
    let available = vec![mode_of(1920, 1080, 60), mode_of(3840, 2160, 144)];

    assert_eq!(choose_mode(mode_of(2560, 1440, 60), &available), None);
    assert_eq!(choose_mode(mode_of(2560, 1440, 60), &[]), None);
}
