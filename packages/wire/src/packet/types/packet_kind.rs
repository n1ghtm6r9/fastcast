#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketKind {
    Hello = 1,
    Welcome = 2,
    Shard = 3,
    Feedback = 4,
    Bye = 5,
    Stats = 6,
    Notice = 7,
}
