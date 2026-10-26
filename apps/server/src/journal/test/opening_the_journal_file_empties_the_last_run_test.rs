use crate::fixtures::TemporaryDirectory;
use crate::journal::open_journal_file;
use std::fs;

#[test]
fn opening_the_journal_file_empties_the_last_run() {
    let directory = TemporaryDirectory::new("journal-last-run");
    let path = directory.write("fastcast.log", b"the last run");

    drop(open_journal_file(&path).expect("the journal file opens"));

    assert_eq!(fs::read(&path).expect("the journal file is read"), b"");
}
