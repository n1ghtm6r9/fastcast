use crate::av1::skip_screen_content_tools;
use crate::avc::BitReader;

pub fn skip_inter_tools(reader: &mut BitReader) -> Option<()> {
    let _enable_interintra_compound = reader.read_bit()?;
    let _enable_masked_compound = reader.read_bit()?;
    let _enable_warped_motion = reader.read_bit()?;
    let _enable_dual_filter = reader.read_bit()?;
    let enable_order_hint = reader.read_bit()? == 1;

    if enable_order_hint {
        let _enable_jnt_comp = reader.read_bit()?;
        let _enable_ref_frame_mvs = reader.read_bit()?;
    }

    skip_screen_content_tools(reader)?;

    if enable_order_hint {
        let _order_hint_bits_minus_1 = reader.read_bits(3)?;
    }

    Some(())
}
