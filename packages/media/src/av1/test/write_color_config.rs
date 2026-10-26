use super::Av1SequenceHeaderParts;
use crate::avc::test::BitWriter;

pub fn write_color_config(writer: &mut BitWriter, parts: &Av1SequenceHeaderParts) {
    writer.write_bit(parts.high_bitdepth);

    if parts.profile == 2 && parts.high_bitdepth {
        writer.write_bit(parts.twelve_bit);
    }

    if parts.profile != 1 {
        writer.write_bit(parts.monochrome);
    }

    writer.write_bit(parts.color_description.is_some());

    for value in parts.color_description.into_iter().flatten() {
        writer.write_bits(value, 8);
    }

    if parts.monochrome {
        writer.write_bit(false);
        return;
    }

    if parts.color_description != Some([1, 13, 0]) {
        writer.write_bit(false);

        let (subsampling_x, subsampling_y) = match parts.profile {
            0 => (true, true),
            1 => (false, false),
            _ if parts.high_bitdepth && parts.twelve_bit => {
                let (subsampling_x, subsampling_y) = parts.twelve_bit_subsampling;

                writer.write_bit(subsampling_x);

                if subsampling_x {
                    writer.write_bit(subsampling_y);
                }

                (subsampling_x, subsampling_x && subsampling_y)
            }
            _ => (true, false),
        };

        if subsampling_x && subsampling_y {
            writer.write_bits(parts.chroma_sample_position, 2);
        }
    }

    writer.write_bit(false);
}
