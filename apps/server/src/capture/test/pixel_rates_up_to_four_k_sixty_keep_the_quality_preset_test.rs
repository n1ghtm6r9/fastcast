use crate::capture::encoder_preset_for;

#[test]
fn pixel_rates_up_to_four_k_sixty_keep_the_quality_preset() {
    assert_eq!(encoder_preset_for(3840, 2160, 60), 4);
    assert_eq!(encoder_preset_for(2560, 1440, 120), 4);
}
