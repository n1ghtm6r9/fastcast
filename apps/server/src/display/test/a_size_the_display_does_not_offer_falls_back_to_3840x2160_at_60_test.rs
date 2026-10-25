use super::mode_of;
use crate::display::{FALLBACK_MODE, choose_mode};

#[test]
fn a_size_the_display_does_not_offer_falls_back_to_3840x2160_at_60() {
    let available = vec![
        mode_of(1920, 1080, 60),
        mode_of(3840, 2160, 240),
        mode_of(3840, 2160, 60),
    ];

    assert_eq!(FALLBACK_MODE, mode_of(3840, 2160, 60));
    assert_eq!(
        choose_mode(mode_of(2560, 1440, 120), &available),
        Some(FALLBACK_MODE)
    );
}
