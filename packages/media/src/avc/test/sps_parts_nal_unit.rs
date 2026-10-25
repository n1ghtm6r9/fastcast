use super::{
    BitWriter, SpsParts, carries_chroma_fields, with_emulation_prevention, write_frame_crop,
    write_high_profile_parts, write_picture_order_count,
};

impl SpsParts {
    pub fn nal_unit(&self) -> Vec<u8> {
        let mut writer = BitWriter::default();

        writer.write_bits(self.profile_idc, 8);
        writer.write_bits(0, 8);
        writer.write_bits(42, 8);
        writer.write_ue(0);

        if carries_chroma_fields(self.profile_idc) {
            write_high_profile_parts(&mut writer, self);
        }

        writer.write_ue(4);
        write_picture_order_count(&mut writer, &self.picture_order_count);
        writer.write_ue(4);
        writer.write_bit(false);
        writer.write_ue(self.width_in_macroblocks - 1);
        writer.write_ue(self.height_in_map_units - 1);
        writer.write_bit(self.frame_mbs_only);

        if !self.frame_mbs_only {
            writer.write_bit(false);
        }

        writer.write_bit(true);
        write_frame_crop(&mut writer, self.crop);
        writer.write_bit(false);

        let mut nal_unit = vec![0x67];

        nal_unit.extend(with_emulation_prevention(&writer.finish()));

        nal_unit
    }
}
