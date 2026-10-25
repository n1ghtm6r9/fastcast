use super::{BitWriter, ScalingListParts};

pub fn write_scaling_list(writer: &mut BitWriter, parts: ScalingListParts, list_index: usize) {
    let size = if list_index < 6 { 16 } else { 64 };

    match parts {
        ScalingListParts::Absent => writer.write_bit(false),
        ScalingListParts::Full => {
            writer.write_bit(true);

            for _ in 0..size {
                writer.write_se(1);
            }
        }
        ScalingListParts::EndsAfter(deltas) => {
            writer.write_bit(true);

            for _ in 0..deltas {
                writer.write_se(1);
            }

            writer.write_se(-(8 + deltas as i64));
        }
    }
}
