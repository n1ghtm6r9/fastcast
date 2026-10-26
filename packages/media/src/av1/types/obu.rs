#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Obu<'a> {
    pub kind: u8,
    pub bytes: &'a [u8],
    pub payload: &'a [u8],
}
