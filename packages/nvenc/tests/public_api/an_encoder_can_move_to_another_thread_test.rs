use nvenc::Encoder;

fn is_send<T: Send>() {}

#[test]
fn an_encoder_can_move_to_another_thread() {
    is_send::<Encoder>();
}
