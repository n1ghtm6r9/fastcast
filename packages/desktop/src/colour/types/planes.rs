#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Planes {
    pub width: usize,
    pub height: usize,
    pub luma: Vec<u16>,
    pub chroma: Vec<u16>,
}
