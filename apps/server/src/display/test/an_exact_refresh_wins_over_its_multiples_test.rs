use super::mode_of;
use crate::display::choose_mode;

#[test]
fn an_exact_refresh_wins_over_its_multiples() {
    let available = vec![
        mode_of(3840, 2160, 240),
        mode_of(3840, 2160, 120),
        mode_of(3840, 2160, 60),
    ];

    assert_eq!(
        choose_mode(mode_of(3840, 2160, 60), &available),
        Some(mode_of(3840, 2160, 60))
    );
}
