use crate::avc::{
    AVCC_FIRST_SPS_LENGTH_OFFSET, AVCC_FIRST_SPS_OFFSET, AVCC_SPS_COUNT_MASK, AVCC_SPS_COUNT_OFFSET,
};

pub fn first_sps_of_avcc(avcc: &[u8]) -> Option<&[u8]> {
    let sps_count = avcc.get(AVCC_SPS_COUNT_OFFSET)? & AVCC_SPS_COUNT_MASK;

    if sps_count == 0 {
        return None;
    }

    let length_bytes = avcc.get(AVCC_FIRST_SPS_LENGTH_OFFSET..AVCC_FIRST_SPS_OFFSET)?;
    let length = usize::from(u16::from_be_bytes(<[u8; 2]>::try_from(length_bytes).ok()?));

    avcc.get(AVCC_FIRST_SPS_OFFSET..AVCC_FIRST_SPS_OFFSET + length)
}
