use crate::avc::VideoSize;
use crate::hevc::HevcSpsFields;

pub fn video_size_from_hevc_sps_fields(fields: &HevcSpsFields) -> Option<VideoSize> {
    let (sub_width, sub_height) = fields.chroma_array_type.crop_unit();
    let window = fields.conformance_window;
    let cropped_columns = u64::from(window.left) + u64::from(window.right);
    let cropped_rows = u64::from(window.top) + u64::from(window.bottom);

    let width = u64::from(fields.width_in_luma_samples).checked_sub(cropped_columns * sub_width)?;
    let height = u64::from(fields.height_in_luma_samples).checked_sub(cropped_rows * sub_height)?;

    if width == 0 || height == 0 {
        return None;
    }

    Some(VideoSize {
        width: u32::try_from(width).ok()?,
        height: u32::try_from(height).ok()?,
    })
}
