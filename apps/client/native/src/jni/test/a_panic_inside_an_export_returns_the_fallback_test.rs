use crate::jni::guarded;

#[test]
fn a_panic_inside_an_export_returns_the_fallback() {
    let result = guarded(7, || -> i32 { panic!("boom") });

    assert_eq!(result, 7);
}
