use std::ops::Range;

pub fn visible_range(offset: i32, cursor_extent: u32, screen_extent: usize) -> Range<usize> {
    let offset = i64::from(offset);
    let screen_extent = i64::try_from(screen_extent).unwrap_or(i64::MAX);
    let start = (-offset).max(0);
    let end = i64::from(cursor_extent)
        .min(screen_extent.saturating_sub(offset))
        .max(start);

    start as usize..end as usize
}
