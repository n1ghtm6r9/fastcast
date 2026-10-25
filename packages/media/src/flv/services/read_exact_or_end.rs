use crate::flv::FlvError;
use std::io::ErrorKind;
use tokio::io::{AsyncRead, AsyncReadExt};

pub async fn read_exact_or_end<R: AsyncRead + Unpin>(
    reader: &mut R,
    buffer: &mut [u8],
) -> Result<bool, FlvError> {
    match reader.read_exact(buffer).await {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == ErrorKind::UnexpectedEof => Ok(false),
        Err(error) => Err(FlvError::Io(error)),
    }
}
