use crate::stream::{LinkJournal, SessionControl};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::Instant;
use wire::{FecController, LossMeter};

pub struct SessionEntry {
    pub id: u32,
    pub nonce: u32,
    pub last_seen: Instant,
    pub control: Arc<SessionControl>,
    pub loss: LossMeter,
    pub fec: FecController,
    pub link: LinkJournal,
    pub thread: JoinHandle<()>,
}
