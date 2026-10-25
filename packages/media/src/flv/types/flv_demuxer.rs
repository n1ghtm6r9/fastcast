use crate::flv::FLV_READ_BUFFER_SIZE;
use tokio::io::{AsyncRead, BufReader};

pub struct FlvDemuxer<R> {
    pub(crate) reader: BufReader<R>,
    pub(crate) header_skipped: bool,
}

impl<R: AsyncRead + Unpin> FlvDemuxer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::with_capacity(FLV_READ_BUFFER_SIZE, reader),
            header_skipped: false,
        }
    }
}
