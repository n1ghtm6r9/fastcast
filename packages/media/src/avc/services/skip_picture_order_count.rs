use crate::avc::BitReader;

pub fn skip_picture_order_count(reader: &mut BitReader) -> Option<()> {
    match reader.read_ue()? {
        0 => {
            let _log2_max_pic_order_cnt_lsb_minus4 = reader.read_ue()?;
        }
        1 => {
            let _delta_pic_order_always_zero_flag = reader.read_bit()?;
            let _offset_for_non_ref_pic = reader.read_se()?;
            let _offset_for_top_to_bottom_field = reader.read_se()?;
            let reference_frames_in_cycle = reader.read_ue()?;

            for _ in 0..reference_frames_in_cycle {
                let _offset_for_ref_frame = reader.read_se()?;
            }
        }
        2 => {}
        _ => return None,
    }

    Some(())
}
