use super::Av1SequenceHeaderParts;
use crate::avc::test::BitWriter;

pub fn write_coding_tools(writer: &mut BitWriter, parts: &Av1SequenceHeaderParts) {
    let full_header = !parts.reduced_still_picture_header;

    if full_header {
        writer.write_bit(parts.frame_id_numbers);

        if parts.frame_id_numbers {
            writer.write_bits(0b1010, 4);
            writer.write_bits(0b101, 3);
        }
    }

    writer.write_bits(0b101, 3);

    if full_header {
        writer.write_bits(0b1010, 4);
        writer.write_bit(parts.order_hint);

        if parts.order_hint {
            writer.write_bits(0b11, 2);
        }

        writer.write_bit(parts.choose_screen_content_tools);

        if !parts.choose_screen_content_tools {
            writer.write_bit(parts.force_screen_content_tools);
        }

        if parts.choose_screen_content_tools || parts.force_screen_content_tools {
            writer.write_bit(parts.choose_integer_mv);

            if !parts.choose_integer_mv {
                writer.write_bit(true);
            }
        }

        if parts.order_hint {
            writer.write_bits(0b110, 3);
        }
    }

    writer.write_bits(0b011, 3);
}
