use crate::gpu::require_even_size;

#[test]
fn an_even_size_is_accepted() {
    assert!(require_even_size(1920, 1080).is_ok());
}
