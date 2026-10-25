use crate::flv::{
    FlvDemuxer, FlvError, FlvEvent, VIDEO_TAG_TYPE, parse_avc_video_tag, read_flv_tag_data,
    read_flv_tag_header, skip_flv_header,
};
use tokio::io::AsyncRead;

impl<R: AsyncRead + Unpin> FlvDemuxer<R> {
    pub async fn next_event(&mut self) -> Result<Option<FlvEvent>, FlvError> {
        if !self.header_skipped {
            if !skip_flv_header(&mut self.reader).await? {
                return Ok(None);
            }

            self.header_skipped = true;
        }

        loop {
            let Some(header) = read_flv_tag_header(&mut self.reader).await? else {
                return Ok(None);
            };

            let Some(data) = read_flv_tag_data(&mut self.reader, header.data_size).await? else {
                return Ok(None);
            };

            if header.kind != VIDEO_TAG_TYPE {
                continue;
            }

            if let Some(event) = parse_avc_video_tag(header.timestamp_ms, data) {
                return Ok(Some(event));
            }
        }
    }
}
