use super::obu_of;

pub fn frame_obu(payload: &[u8]) -> Vec<u8> {
    obu_of(6, payload)
}
