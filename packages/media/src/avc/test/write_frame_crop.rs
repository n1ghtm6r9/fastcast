use super::BitWriter;

pub fn write_frame_crop(writer: &mut BitWriter, crop: Option<[u32; 4]>) {
    let Some(offsets) = crop else {
        writer.write_bit(false);
        return;
    };

    writer.write_bit(true);

    for offset in offsets {
        writer.write_ue(offset);
    }
}
