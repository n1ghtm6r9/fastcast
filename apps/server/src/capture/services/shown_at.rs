use desktop::present_age;
use std::time::Instant;

pub fn shown_at(presented: i64, now: Instant) -> Option<Instant> {
    present_age(presented).and_then(|age| now.checked_sub(age))
}
