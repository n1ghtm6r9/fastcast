pub fn desktop_extent(start: i32, end: i32) -> u32 {
    (i64::from(end) - i64::from(start)).clamp(0, i64::from(u32::MAX)) as u32
}
