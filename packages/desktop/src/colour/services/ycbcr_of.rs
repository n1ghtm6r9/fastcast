use crate::colour::{
    BLUE_DIFFERENCE_SCALE, BT709_BLUE_WEIGHT, BT709_GREEN_WEIGHT, BT709_RED_WEIGHT, BitDepth,
    RED_DIFFERENCE_SCALE, YCbCr, round_half_up,
};

pub fn ycbcr_of(r: f32, g: f32, b: f32, depth: BitDepth) -> YCbCr {
    let luma = BT709_RED_WEIGHT * r + BT709_GREEN_WEIGHT * g + BT709_BLUE_WEIGHT * b;
    let blue_difference = (b - luma) / BLUE_DIFFERENCE_SCALE;
    let red_difference = (r - luma) / RED_DIFFERENCE_SCALE;

    YCbCr {
        y: round_half_up(depth.luma_black() + depth.luma_range() * luma),
        cb: round_half_up(depth.chroma_centre() + depth.chroma_range() * blue_difference),
        cr: round_half_up(depth.chroma_centre() + depth.chroma_range() * red_difference),
    }
}
