use crate::annexb::START_CODE;

pub fn start_code_position(data: &[u8], from: usize) -> Option<usize> {
    let offset = data
        .get(from..)?
        .windows(START_CODE.len())
        .position(|window| window == START_CODE)?;

    Some(from + offset)
}
