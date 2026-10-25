use crate::gpu::list_outputs;

#[test]
#[ignore = "needs a GPU and a desktop"]
fn the_desktop_has_an_attached_output() {
    let outputs = list_outputs().unwrap();

    assert!(
        outputs
            .iter()
            .any(|output| output.attached && output.width > 0 && output.height > 0)
    );
}
