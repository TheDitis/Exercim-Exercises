use std::process::Command;
use std::{fs, io, thread};
use std::fs::DirEntry;

fn main() {
    // get all directories in the Exercism folder
    let paths = fs::read_dir("../").unwrap();
    let mut handles = vec![];
    println!("checking projects...\n");

    // for each directory, spawn a new thread, run clippy in that dir & print the results if there are any
    for path in paths {
        let path = parse_dir_path(path);
        let handle = thread::spawn(move || {
            let clippy_res = run_clippy(&path);
            if has_problems(clippy_res.as_str()) {
                println!("{} has problems", &path);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("\nDone checking projects")
}

fn parse_dir_path(dir_entry: io::Result<DirEntry>) -> String {
    dir_entry.unwrap().path().display().to_string()
}

fn run_clippy(dir_path: &str) -> String {
    let file_path = format!("{}/Cargo.toml", dir_path);
    let stderr = Command::new("cargo")
        .args(["clippy", "--all", "--manifest-path", file_path.as_str()])
        .output()
        .expect("error running clippy")
        .stderr;
    String::from_utf8(stderr).unwrap()
}

fn has_problems(clippy_output: &str) -> bool {
    static CLIPPY_WAITING: &str = "    Blocking waiting for file lock on package cache";
    clippy_output.lines().filter(|&l| l != CLIPPY_WAITING).count() > 3
}
