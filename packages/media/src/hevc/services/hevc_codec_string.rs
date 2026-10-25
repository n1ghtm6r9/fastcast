use crate::hevc::{
    HVCC_COMPATIBILITY_FLAGS_OFFSET, HVCC_CONSTRAINT_FLAGS_OFFSET, HVCC_LEVEL_OFFSET,
    HVCC_PROFILE_IDC_MASK, HVCC_PROFILE_OFFSET, HVCC_PROFILE_SPACE_SHIFT, HVCC_TIER_FLAG_MASK,
    constraint_flags_suffix, profile_space_prefix,
};

pub fn hevc_codec_string(hvcc: &[u8]) -> Option<String> {
    let profile = *hvcc.get(HVCC_PROFILE_OFFSET)?;
    let compatibility_bytes =
        hvcc.get(HVCC_COMPATIBILITY_FLAGS_OFFSET..HVCC_CONSTRAINT_FLAGS_OFFSET)?;
    let constraint_bytes = hvcc.get(HVCC_CONSTRAINT_FLAGS_OFFSET..HVCC_LEVEL_OFFSET)?;
    let level = *hvcc.get(HVCC_LEVEL_OFFSET)?;

    let compatibility = u32::from_be_bytes(<[u8; 4]>::try_from(compatibility_bytes).ok()?);
    let tier = if profile & HVCC_TIER_FLAG_MASK == 0 {
        'L'
    } else {
        'H'
    };

    Some(format!(
        "hvc1.{}{}.{:X}.{}{}{}",
        profile_space_prefix(profile >> HVCC_PROFILE_SPACE_SHIFT),
        profile & HVCC_PROFILE_IDC_MASK,
        compatibility.reverse_bits(),
        tier,
        level,
        constraint_flags_suffix(constraint_bytes),
    ))
}
