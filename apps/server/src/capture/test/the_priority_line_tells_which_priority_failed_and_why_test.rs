use crate::capture::priority_line;

#[test]
fn the_priority_line_tells_which_priority_failed_and_why() {
    let line = priority_line(&[
        ("process above normal", Ok(())),
        ("GPU scheduling high", Err(String::from("access denied"))),
    ]);

    assert_eq!(
        line,
        "capture priorities: process above normal, GPU scheduling high failed: access denied"
    );
}
