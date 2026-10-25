use super::{BitWriter, SpsParts, write_scaling_list};

pub fn write_high_profile_parts(writer: &mut BitWriter, parts: &SpsParts) {
    writer.write_ue(parts.chroma_format_idc);

    if parts.chroma_format_idc == 3 {
        writer.write_bit(parts.separate_colour_plane);
    }

    writer.write_ue(2);
    writer.write_ue(2);
    writer.write_bit(false);

    match &parts.scaling_matrix {
        None => writer.write_bit(false),
        Some(lists) => {
            writer.write_bit(true);

            for (list_index, list) in lists.iter().enumerate() {
                write_scaling_list(writer, *list, list_index);
            }
        }
    }
}
