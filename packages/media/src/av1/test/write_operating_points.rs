use super::Av1SequenceHeaderParts;
use crate::avc::test::BitWriter;

pub fn write_operating_points(writer: &mut BitWriter, parts: &Av1SequenceHeaderParts) {
    let decoder_model = parts.timing_info.and(parts.buffer_delay_length);

    writer.write_bit(parts.timing_info.is_some());

    if let Some(equal_picture_interval) = parts.timing_info {
        writer.write_bits(1001, 32);
        writer.write_bits(60_000, 32);
        writer.write_bit(equal_picture_interval);

        if equal_picture_interval {
            writer.write_ue(4);
        }

        writer.write_bit(decoder_model.is_some());

        if let Some(length) = decoder_model {
            writer.write_bits(length - 1, 5);
            writer.write_bits(90_000, 32);
            writer.write_bits(23, 5);
            writer.write_bits(9, 5);
        }
    }

    writer.write_bit(parts.initial_display_delay);
    writer.write_bits(parts.operating_points.len() as u32 - 1, 5);

    for &(level, tier) in &parts.operating_points {
        writer.write_bits(0x101, 12);
        writer.write_bits(level, 5);

        if level > 7 {
            writer.write_bit(tier);
        }

        if let Some(length) = decoder_model {
            writer.write_bit(true);
            writer.write_bits((1 << length) - 1, length);
            writer.write_bits((1 << length) - 1, length);
            writer.write_bit(true);
        }

        if parts.initial_display_delay {
            writer.write_bit(true);
            writer.write_bits(9, 4);
        }
    }
}
