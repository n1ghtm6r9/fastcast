use crate::flv::{
    FLV_TAG_HEADER_LENGTH, FlvError, FlvTagHeader, parse_flv_tag_header, read_exact_or_end,
};
use tokio::io::AsyncRead;

pub async fn read_flv_tag_header<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<Option<FlvTagHeader>, FlvError> {
    let mut bytes = [0u8; FLV_TAG_HEADER_LENGTH];

    if !read_exact_or_end(reader, &mut bytes).await? {
        return Ok(None);
    }

    Ok(Some(parse_flv_tag_header(&bytes)))
}
