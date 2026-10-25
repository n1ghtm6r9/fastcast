use super::TemporaryDirectory;
use crate::app::find_dotenv_file;
use std::fs;

#[test]
fn the_nearest_dotenv_up_the_directory_tree_is_found() {
    let directory = TemporaryDirectory::new("the_nearest_dotenv");
    let outer = directory.write(".env", b"A=1\n");
    let nested = directory.path().join("apps").join("server");
    fs::create_dir_all(&nested).expect("the nested directory can be created");

    assert_eq!(find_dotenv_file(&nested), Some(outer));

    let inner = directory.write("apps/.env", b"A=2\n");

    assert_eq!(find_dotenv_file(&nested), Some(inner));
}
