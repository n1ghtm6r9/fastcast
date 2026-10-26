use crate::link::test::FakeServer;
use wire::Packetizer;

impl FakeServer {
    pub(crate) fn packetizer(&mut self) -> &mut Packetizer {
        self.packetizer
            .as_mut()
            .expect("the fake server has not welcomed anyone yet")
    }
}
