use crate::avc::{
    BitReader, FIRST_LARGE_SCALING_LIST, LARGE_SCALING_LIST_SIZE, SMALL_SCALING_LIST_SIZE,
    skip_scaling_list,
};

pub fn skip_scaling_matrix(reader: &mut BitReader, list_count: usize) -> Option<()> {
    for list_index in 0..list_count {
        let list_present = reader.read_bit()? == 1;

        if !list_present {
            continue;
        }

        let size = if list_index < FIRST_LARGE_SCALING_LIST {
            SMALL_SCALING_LIST_SIZE
        } else {
            LARGE_SCALING_LIST_SIZE
        };

        skip_scaling_list(reader, size)?;
    }

    Some(())
}
