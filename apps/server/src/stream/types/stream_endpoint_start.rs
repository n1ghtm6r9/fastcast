use crate::hub::Hub;
use crate::stream::{EndpointOptions, READ_TIMEOUT, StreamEndpoint, StreamError, run_receiver};
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread::Builder;

impl StreamEndpoint {
    pub fn start(
        socket: UdpSocket,
        hub: Arc<Hub>,
        options: EndpointOptions,
    ) -> Result<StreamEndpoint, StreamError> {
        socket
            .set_read_timeout(Some(READ_TIMEOUT))
            .map_err(|source| StreamError::Socket { source })?;

        let local = socket
            .local_addr()
            .map_err(|source| StreamError::Socket { source })?;
        let socket = Arc::new(socket);
        let stop = Arc::new(AtomicBool::new(false));
        let receiver = Builder::new()
            .name("fastcast-stream".to_owned())
            .spawn({
                let stop = stop.clone();

                move || run_receiver(socket, hub, options, stop)
            })
            .map_err(|source| StreamError::Socket { source })?;

        Ok(StreamEndpoint {
            local,
            stop,
            receiver: Some(receiver),
        })
    }
}
