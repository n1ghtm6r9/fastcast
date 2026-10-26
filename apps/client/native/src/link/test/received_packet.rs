use wire::{Feedback, Hello};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ReceivedPacket {
    Hello { session: u32, hello: Hello },
    Feedback { session: u32, feedback: Feedback },
    Bye { session: u32 },
    Stats { session: u32, text: String },
}
