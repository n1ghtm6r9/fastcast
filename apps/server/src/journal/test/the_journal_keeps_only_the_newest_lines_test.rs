use crate::journal::JournalLines;

#[test]
fn the_journal_keeps_only_the_newest_lines() {
    let lines = JournalLines::new(3);

    for number in 1..=5 {
        lines.push(format!("line {number}"));
    }

    assert_eq!(lines.lines(), ["line 3", "line 4", "line 5"]);
}
