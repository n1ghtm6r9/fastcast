use crate::avc::BitReader;
use crate::hevc::{
    MAX_SUB_LAYERS, PROFILE_TIER_LEVEL_LEVEL_BITS, PROFILE_TIER_LEVEL_PROFILE_BITS,
    SUB_LAYER_PADDING_BITS, skip_bits,
};

pub fn skip_profile_tier_level(reader: &mut BitReader, max_sub_layers_minus1: u32) -> Option<()> {
    skip_bits(reader, PROFILE_TIER_LEVEL_PROFILE_BITS)?;
    skip_bits(reader, PROFILE_TIER_LEVEL_LEVEL_BITS)?;

    let sub_layer_count = usize::try_from(max_sub_layers_minus1).ok()?;
    let mut presence = [(false, false); MAX_SUB_LAYERS];

    for sub_layer in presence.iter_mut().take(sub_layer_count) {
        let profile_present = reader.read_bit()? == 1;
        let level_present = reader.read_bit()? == 1;

        *sub_layer = (profile_present, level_present);
    }

    if sub_layer_count > 0 {
        let padding_pairs = MAX_SUB_LAYERS.checked_sub(sub_layer_count)?;

        skip_bits(reader, SUB_LAYER_PADDING_BITS * padding_pairs)?;
    }

    for (profile_present, level_present) in presence.into_iter().take(sub_layer_count) {
        if profile_present {
            skip_bits(reader, PROFILE_TIER_LEVEL_PROFILE_BITS)?;
        }

        if level_present {
            skip_bits(reader, PROFILE_TIER_LEVEL_LEVEL_BITS)?;
        }
    }

    Some(())
}
