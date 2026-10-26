use crate::link::LinkEvent;

#[test]
fn event_kinds_match_the_kotlin_constants() {
    let kinds = [
        LinkEvent::Video(Vec::new()).kind(),
        LinkEvent::Config(Vec::new()).kind(),
        LinkEvent::Connected.kind(),
        LinkEvent::Ended(String::new()).kind(),
    ];

    assert_eq!(kinds, [0, 1, 2, 3]);
    assert_eq!(
        LinkEvent::Ended("Closed by server".to_string()).payload(),
        b"Closed by server"
    );
    assert!(LinkEvent::Connected.payload().is_empty());
}
