use super::{HevcSpsParts, write_profile_tier_level};
use crate::avc::test::{BitWriter, with_emulation_prevention, write_frame_crop};

impl HevcSpsParts {
    pub fn nal_unit(&self) -> Vec<u8> {
        let mut writer = BitWriter::default();

        writer.write_bits(0, 4);
        writer.write_bits(self.sub_layers.len() as u32, 3);
        writer.write_bit(true);
        write_profile_tier_level(&mut writer, self);
        writer.write_ue(0);
        writer.write_ue(self.chroma_format_idc);

        if self.chroma_format_idc == 3 {
            writer.write_bit(self.separate_colour_plane);
        }

        writer.write_ue(self.width_in_luma_samples);
        writer.write_ue(self.height_in_luma_samples);
        write_frame_crop(&mut writer, self.conformance_window);
        writer.write_ue(0);
        writer.write_ue(0);
        writer.write_ue(4);

        let mut nal_unit = vec![0x42, 0x01];

        nal_unit.extend(with_emulation_prevention(&writer.finish()));

        nal_unit
    }
}
