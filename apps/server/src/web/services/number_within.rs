use std::ops::RangeInclusive;

pub fn number_within(text: &str, range: &RangeInclusive<u32>) -> Option<u32> {
    text.trim()
        .parse()
        .ok()
        .filter(|number| range.contains(number))
}
