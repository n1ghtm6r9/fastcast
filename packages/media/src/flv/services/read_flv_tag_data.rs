use crate::flv::{FLV_PREVIOUS_TAG_SIZE_LENGTH, FlvError, read_exact_or_end};
use bytes::{Bytes, BytesMut};
use tokio::io::AsyncRead;

pub async fn read_flv_tag_data<R: AsyncRead + Unpin>(
    reader: &mut R,
    data_size: u32,
) -> Result<Option<Bytes>, FlvError> {
    let mut data = BytesMut::zeroed(data_size as usize);

    if !read_exact_or_end(reader, &mut data).await? {
        return Ok(None);
    }

    let mut previous_tag_size = [0u8; FLV_PREVIOUS_TAG_SIZE_LENGTH];

    if !read_exact_or_end(reader, &mut previous_tag_size).await? {
        return Ok(None);
    }

    Ok(Some(data.freeze()))
}
