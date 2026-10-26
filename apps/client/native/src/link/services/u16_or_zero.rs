pub(crate) fn u16_or_zero(value: i32) -> u16 {
    u16::try_from(value).unwrap_or(0)
}
