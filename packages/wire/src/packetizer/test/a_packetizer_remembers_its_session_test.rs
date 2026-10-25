use crate::packetizer::Packetizer;

#[test]
fn a_packetizer_remembers_its_session() {
    assert_eq!(Packetizer::new(0xABCD_1234).session(), 0xABCD_1234);
}
