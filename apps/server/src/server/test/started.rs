use crate::server::ServerStatus;
use crate::server::test::FakeEvent;

pub fn started(name: &str) -> FakeEvent {
    FakeEvent::Started {
        name: name.to_owned(),
        status: ServerStatus::default(),
    }
}
