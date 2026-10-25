use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=native");
    println!("cargo:rerun-if-changed=vendor");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    let mut sources: Vec<PathBuf> = fs::read_dir("native")
        .expect("the native directory is next to build.rs")
        .map(|entry| entry.expect("a native directory entry is readable").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "c"))
        .collect();
    sources.sort();

    cc::Build::new()
        .files(&sources)
        .include("native")
        .include("vendor")
        .flag("/W4")
        .warnings_into_errors(true)
        .compile("fastcast_nvenc");
}
