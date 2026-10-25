use crate::flv::{FlvDemuxer, FlvEvent};
use std::io::Cursor;

pub async fn demux_all(stream: Vec<u8>) -> Vec<FlvEvent> {
    let mut demuxer = FlvDemuxer::new(Cursor::new(stream));
    let mut events = Vec::new();

    while let Some(event) = demuxer
        .next_event()
        .await
        .expect("a well formed stream demuxes")
    {
        events.push(event);
    }

    events
}
