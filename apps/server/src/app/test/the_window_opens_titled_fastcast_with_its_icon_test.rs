use crate::app::window_options;

#[test]
fn the_window_opens_titled_fastcast_with_its_icon() {
    let options = window_options();

    assert_eq!(options.viewport.title.as_deref(), Some("fastcast"));
    assert!(options.viewport.icon.is_some());
}
