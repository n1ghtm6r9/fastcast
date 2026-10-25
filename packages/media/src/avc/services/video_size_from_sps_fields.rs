use crate::avc::{MACROBLOCK_SIZE, SpsFields, VideoSize};

pub fn video_size_from_sps_fields(fields: &SpsFields) -> Option<VideoSize> {
    let fields_per_frame: u64 = if fields.frame_mbs_only { 1 } else { 2 };
    let (crop_unit_x, base_crop_unit_y) = fields.chroma_array_type.crop_unit();
    let crop_unit_y = base_crop_unit_y * fields_per_frame;

    let coded_width = u64::from(fields.width_in_macroblocks) * MACROBLOCK_SIZE;
    let coded_height = u64::from(fields.height_in_map_units) * MACROBLOCK_SIZE * fields_per_frame;
    let cropped_columns = u64::from(fields.crop.left) + u64::from(fields.crop.right);
    let cropped_rows = u64::from(fields.crop.top) + u64::from(fields.crop.bottom);

    let width = coded_width.checked_sub(cropped_columns * crop_unit_x)?;
    let height = coded_height.checked_sub(cropped_rows * crop_unit_y)?;

    if width == 0 || height == 0 {
        return None;
    }

    Some(VideoSize {
        width: u32::try_from(width).ok()?,
        height: u32::try_from(height).ok()?,
    })
}
