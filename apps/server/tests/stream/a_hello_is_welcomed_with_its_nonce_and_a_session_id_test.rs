use crate::support::{TestViewer, a_bare_hello, hub_without_config, start_endpoint};
use wire::{SESSION_TIMEOUT, Welcome};

#[test]
fn a_hello_is_welcomed_with_its_nonce_and_a_session_id() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);
    let hello = a_bare_hello();
    let viewer = TestViewer::connect(endpoint.local_addr(), hello.clone());

    assert_eq!(
        viewer.welcome(),
        Welcome {
            nonce: hello.nonce,
            shard_size: 1370
        }
    );
    assert_ne!(viewer.session(), 0);
}
