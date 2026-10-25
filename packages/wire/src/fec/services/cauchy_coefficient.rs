use crate::fec::MAX_DATA_SHARDS;
use crate::gf::gf_inv;

pub fn cauchy_coefficient(parity_row: usize, data_column: usize) -> u8 {
    gf_inv((MAX_DATA_SHARDS + parity_row) as u8 ^ data_column as u8)
}
