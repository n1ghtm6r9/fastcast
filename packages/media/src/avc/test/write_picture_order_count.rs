use super::{BitWriter, PictureOrderCountParts};

pub fn write_picture_order_count(writer: &mut BitWriter, parts: &PictureOrderCountParts) {
    match parts {
        PictureOrderCountParts::Type0 => {
            writer.write_ue(0);
            writer.write_ue(4);
        }
        PictureOrderCountParts::Type1 { reference_offsets } => {
            writer.write_ue(1);
            writer.write_bit(false);
            writer.write_se(-3);
            writer.write_se(5);
            writer.write_ue(reference_offsets.len() as u32);

            for offset in reference_offsets {
                writer.write_se(*offset);
            }
        }
        PictureOrderCountParts::Type2 => writer.write_ue(2),
        PictureOrderCountParts::Unknown(kind) => writer.write_ue(*kind),
    }
}
