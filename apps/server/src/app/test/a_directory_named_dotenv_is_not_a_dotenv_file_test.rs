use crate::app::find_dotenv_file;
use crate::fixtures::TemporaryDirectory;
use std::fs;

#[test]
fn a_directory_named_dotenv_is_not_a_dotenv_file() {
    let directory = TemporaryDirectory::new("a_directory_named_dotenv");
    let file = directory.write(".env", b"A=1\n");
    let virtual_environment = directory.path().join("python");
    fs::create_dir_all(virtual_environment.join(".env"))
        .expect("a directory can be named like the file");

    assert_eq!(find_dotenv_file(&virtual_environment), Some(file));
}
