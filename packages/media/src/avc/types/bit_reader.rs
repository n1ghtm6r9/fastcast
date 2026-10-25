pub struct BitReader<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) position: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }
}
