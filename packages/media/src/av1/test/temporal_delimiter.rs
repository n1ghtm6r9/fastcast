use super::obu_of;

pub fn temporal_delimiter() -> Vec<u8> {
    obu_of(2, &[])
}
