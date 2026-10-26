use crate::gf::product_row;

pub(crate) fn mul_add_slice_scalar(coefficient: u8, source: &[u8], target: &mut [u8]) {
    match coefficient {
        0 => {}
        1 => {
            for (target_byte, source_byte) in target.iter_mut().zip(source) {
                *target_byte ^= source_byte;
            }
        }
        _ => {
            let row = product_row(coefficient);

            for (target_byte, source_byte) in target.iter_mut().zip(source) {
                *target_byte ^= row[*source_byte as usize];
            }
        }
    }
}
