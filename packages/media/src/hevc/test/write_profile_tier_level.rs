use super::{HevcSpsParts, write_profile_bits};
use crate::avc::test::BitWriter;

pub fn write_profile_tier_level(writer: &mut BitWriter, parts: &HevcSpsParts) {
    write_profile_bits(writer);
    writer.write_bits(120, 8);

    for sub_layer in &parts.sub_layers {
        writer.write_bit(sub_layer.profile_present);
        writer.write_bit(sub_layer.level_present);
    }

    if !parts.sub_layers.is_empty() {
        for _ in parts.sub_layers.len()..8 {
            writer.write_bits(0, 2);
        }
    }

    for sub_layer in &parts.sub_layers {
        if sub_layer.profile_present {
            write_profile_bits(writer);
        }

        if sub_layer.level_present {
            writer.write_bits(90, 8);
        }
    }
}
