use crate::hub::Hub;

#[test]
fn a_scale_ask_reaches_every_watcher() {
    let hub = Hub::new();
    let mut first = hub.watch_scale_asks();
    let mut second = hub.watch_scale_asks();

    hub.ask_scale(1);
    hub.ask_scale(-1);

    assert_eq!((first.try_recv(), first.try_recv()), (Ok(1), Ok(-1)));
    assert_eq!((second.try_recv(), second.try_recv()), (Ok(1), Ok(-1)));
}
