use crate::fixtures::TemporaryDirectory;
use crate::journal::open_journal_file;

#[test]
fn opening_the_journal_file_creates_its_directory() {
    let directory = TemporaryDirectory::new("journal-directory");
    let path = directory.path().join("fastcast").join("fastcast.log");

    drop(open_journal_file(&path).expect("the journal file opens"));

    assert!(path.is_file());
}
