#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkEvent {
    Connected,
    Config(Vec<u8>),
    Video(Vec<u8>),
    Ended(String),
}
