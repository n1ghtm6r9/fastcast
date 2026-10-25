use crate::encoder::NVENC_CALL_NAMES;

pub fn nvenc_call_name(call: u32) -> &'static str {
    call.checked_sub(1)
        .and_then(|index| NVENC_CALL_NAMES.get(index as usize))
        .copied()
        .unwrap_or("NVENC")
}
