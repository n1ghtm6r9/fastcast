use super::{
    Av1SequenceHeaderParts, obu_of, write_coding_tools, write_color_config, write_operating_points,
};
use crate::avc::test::BitWriter;

impl Av1SequenceHeaderParts {
    pub fn obu(&self) -> Vec<u8> {
        let mut writer = BitWriter::default();

        writer.write_bits(self.profile, 3);
        writer.write_bit(self.reduced_still_picture_header);
        writer.write_bit(self.reduced_still_picture_header);

        if self.reduced_still_picture_header {
            writer.write_bits(self.operating_points[0].0, 5);
        } else {
            write_operating_points(&mut writer, self);
        }

        writer.write_bits(15, 4);
        writer.write_bits(15, 4);
        writer.write_bits(self.width - 1, 16);
        writer.write_bits(self.height - 1, 16);
        write_coding_tools(&mut writer, self);
        write_color_config(&mut writer, self);
        writer.write_bit(false);

        obu_of(1, &writer.finish())
    }
}
