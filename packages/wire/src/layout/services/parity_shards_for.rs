use crate::fec::MAX_PARITY_SHARDS;

pub fn parity_shards_for(data_shards: usize, fec_percent: u8) -> usize {
    if fec_percent == 0 {
        return 0;
    }

    data_shards
        .saturating_mul(fec_percent as usize)
        .div_ceil(100)
        .clamp(1, MAX_PARITY_SHARDS)
}
