#[derive(Debug, Clone)]
pub enum PictureOrderCountParts {
    Type0,
    Type1 { reference_offsets: Vec<i64> },
    Type2,
    Unknown(u32),
}
