use crate::av1::{skip_frame_id_numbers, skip_inter_tools};
use crate::avc::BitReader;

pub fn skip_coding_tools(reader: &mut BitReader, reduced_still_picture_header: bool) -> Option<()> {
    if !reduced_still_picture_header {
        skip_frame_id_numbers(reader)?;
    }

    let _use_128x128_superblock = reader.read_bit()?;
    let _enable_filter_intra = reader.read_bit()?;
    let _enable_intra_edge_filter = reader.read_bit()?;

    if !reduced_still_picture_header {
        skip_inter_tools(reader)?;
    }

    let _enable_superres = reader.read_bit()?;
    let _enable_cdef = reader.read_bit()?;
    let _enable_restoration = reader.read_bit()?;

    Some(())
}
