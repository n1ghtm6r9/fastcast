#[derive(Debug, Default)]
pub struct Datagrams {
    pub(crate) bytes: Vec<u8>,
    pub(crate) ends: Vec<usize>,
}
