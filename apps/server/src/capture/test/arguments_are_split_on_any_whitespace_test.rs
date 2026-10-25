use crate::capture::args_of;

#[test]
fn arguments_are_split_on_any_whitespace() {
    assert_eq!(args_of("  a   b\tc \n"), vec!["a", "b", "c"]);
    assert!(args_of("   ").is_empty());
}
