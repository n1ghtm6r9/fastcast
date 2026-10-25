use crate::flv::{
    FLV_HEADER_LENGTH, FLV_PREVIOUS_TAG_SIZE_LENGTH, FLV_SIGNATURE, FlvError,
    LARGEST_FLV_HEADER_LENGTH, read_exact_or_end,
};
use tokio::io::AsyncRead;

pub async fn skip_flv_header<R: AsyncRead + Unpin>(reader: &mut R) -> Result<bool, FlvError> {
    let mut header = [0u8; FLV_HEADER_LENGTH];

    if !read_exact_or_end(reader, &mut header).await? {
        return Ok(false);
    }

    if header[..FLV_SIGNATURE.len()] != FLV_SIGNATURE {
        return Err(FlvError::InvalidHeader);
    }

    let declared_length = u32::from_be_bytes([header[5], header[6], header[7], header[8]]) as usize;

    if !(FLV_HEADER_LENGTH..=LARGEST_FLV_HEADER_LENGTH).contains(&declared_length) {
        return Err(FlvError::InvalidHeader);
    }

    let mut rest = vec![0u8; declared_length - FLV_HEADER_LENGTH + FLV_PREVIOUS_TAG_SIZE_LENGTH];

    read_exact_or_end(reader, &mut rest).await
}
