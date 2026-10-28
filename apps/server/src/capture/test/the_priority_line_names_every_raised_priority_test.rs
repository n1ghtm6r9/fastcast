use crate::capture::priority_line;

#[test]
fn the_priority_line_names_every_raised_priority() {
    let line = priority_line(&[
        ("process above normal", Ok(())),
        ("capture thread highest", Ok(())),
    ]);

    assert_eq!(
        line,
        "capture priorities: process above normal, capture thread highest"
    );
}
