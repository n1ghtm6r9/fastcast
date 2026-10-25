use crate::packet::{Feedback, Hello, Shard, Welcome};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet<'a> {
    Hello(Hello),
    Welcome(Welcome),
    Shard(Shard<'a>),
    Feedback(Feedback),
    Bye,
    Stats(&'a str),
}
