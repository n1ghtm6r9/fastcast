use crate::av1::{
    AV1C_LEVEL_MASK, AV1C_MARKER_AND_VERSION, AV1C_PROFILE_SHIFT, AV1C_TIER_SHIFT, av1_bit_depth,
};

pub fn av1_codec_string(av1c: &[u8]) -> Option<String> {
    let &[AV1C_MARKER_AND_VERSION, profile_and_level, flags, ..] = av1c else {
        return None;
    };

    let profile = profile_and_level >> AV1C_PROFILE_SHIFT;
    let level = profile_and_level & AV1C_LEVEL_MASK;
    let tier = if (flags >> AV1C_TIER_SHIFT) & 1 == 1 {
        'H'
    } else {
        'M'
    };

    Some(format!(
        "av01.{profile}.{level:02}{tier}.{:02}",
        av1_bit_depth(flags)
    ))
}
