use crate::colour::CHANNEL_MAX;

pub fn unit_rgb(bgra: &[u8], width: usize, column: usize, row: usize) -> [f32; 3] {
    let offset = (row * width + column) * 4;
    let blue = f32::from(bgra[offset]);
    let green = f32::from(bgra[offset + 1]);
    let red = f32::from(bgra[offset + 2]);

    [red / CHANNEL_MAX, green / CHANNEL_MAX, blue / CHANNEL_MAX]
}
