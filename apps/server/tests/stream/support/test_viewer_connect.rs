use crate::support::{
    CONNECT_LIMIT, HELLO_RESEND, TestViewer, bind_peer, encoded_datagram, receive_within,
};
use std::net::SocketAddr;
use std::time::Instant;
use wire::{FrameReceiver, Hello, Packet, ReceiveCounters, decode_datagram};

impl TestViewer {
    pub fn connect(server: SocketAddr, hello: Hello) -> TestViewer {
        let socket = bind_peer();
        let packet = encoded_datagram(0, 0, &Packet::Hello(hello.clone()));
        let give_up = Instant::now() + CONNECT_LIMIT;

        while Instant::now() < give_up {
            socket.send_to(&packet, server).expect("the hello is sent");

            let resend = Instant::now() + HELLO_RESEND;

            while let Some((bytes, _)) =
                receive_within(&socket, resend.saturating_duration_since(Instant::now()))
            {
                let Some(datagram) = decode_datagram(&bytes) else {
                    continue;
                };
                let Packet::Welcome(welcome) = datagram.packet else {
                    continue;
                };

                if welcome.nonce == hello.nonce {
                    let mut counters = ReceiveCounters::default();

                    counters.record(datagram.sequence);

                    return TestViewer {
                        socket,
                        server,
                        session: datagram.session,
                        welcome,
                        next_sequence: 0,
                        counters,
                        receiver: FrameReceiver::new(),
                    };
                }
            }
        }

        panic!("the server welcomes the hello within {CONNECT_LIMIT:?}");
    }
}
