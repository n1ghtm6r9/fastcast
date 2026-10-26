use crate::stream::{StreamError, tune_stream_socket};
use std::net::{SocketAddr, UdpSocket};

pub fn bind_stream_socket(address: SocketAddr) -> Result<UdpSocket, StreamError> {
    let socket =
        UdpSocket::bind(address).map_err(|source| StreamError::Bind { address, source })?;

    tune_stream_socket(&socket).map_err(|source| StreamError::Socket { source })?;

    Ok(socket)
}
