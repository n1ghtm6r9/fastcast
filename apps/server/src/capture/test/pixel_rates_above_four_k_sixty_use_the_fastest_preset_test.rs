use crate::capture::encoder_preset_for;

#[test]
fn pixel_rates_above_four_k_sixty_use_the_fastest_preset() {
    assert_eq!(encoder_preset_for(3840, 2160, 120), 1);
    assert_eq!(encoder_preset_for(7680, 4320, 240), 1);
}
