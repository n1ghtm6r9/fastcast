use super::mode_of;
use crate::display::choose_mode;

#[test]
fn an_exact_mode_is_chosen_when_the_display_offers_it() {
    let available = vec![
        mode_of(1920, 1080, 60),
        mode_of(3840, 2160, 144),
        mode_of(3840, 2160, 60),
        mode_of(3840, 2160, 30),
    ];

    assert_eq!(
        choose_mode(mode_of(3840, 2160, 60), &available),
        Some(mode_of(3840, 2160, 60))
    );
}
