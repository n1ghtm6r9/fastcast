#[derive(Debug, Clone, Copy)]
pub enum ScalingListParts {
    Absent,
    Full,
    EndsAfter(usize),
}
