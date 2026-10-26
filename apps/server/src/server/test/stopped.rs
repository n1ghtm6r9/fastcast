use crate::server::test::FakeEvent;

pub fn stopped(name: &str) -> FakeEvent {
    FakeEvent::Stopped {
        name: name.to_owned(),
    }
}
