pub fn round_half_up(value: f32) -> u16 {
    (value + 0.5).floor() as u16
}
