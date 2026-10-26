#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessUnit {
    pub record: Option<Vec<u8>>,
    pub payload: Vec<u8>,
}
