use crate::window::ServerWindow;
use egui_kittest::Harness;

pub fn window_harness(window: ServerWindow<'static>) -> Harness<'static, ServerWindow<'static>> {
    let mut harness = Harness::builder()
        .with_size([960.0, 1400.0])
        .build_ui_state(
            |ui, window: &mut ServerWindow<'static>| window.show(ui),
            window,
        );

    harness.run();
    harness
}
