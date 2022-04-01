use std::process::Command;
use std::{fs, io};
use std::fs::DirEntry;

fn main() {
    println!("checking projects...");
    let paths = fs::read_dir("../").unwrap();
    let need_cleanup: Vec<String> = paths
        .map(parse_dir_path)
        .map(run_clippy)
        .filter(|(_, clippy_results)| has_problems(&clippy_results))
        .map(proj_name_only).collect();

    print_results(need_cleanup);
}

fn parse_dir_path(dir_entry: io::Result<DirEntry>) -> String {
    dir_entry.unwrap().path().display().to_string()
}

fn run_clippy(dir_path: String) -> (String, String) {
    let file_path = format!("{}/Cargo.toml", dir_path);
    let stderr = Command::new("cargo")
        .args(["clippy", "--all", "--manifest-path", file_path.as_str()])
        .output()
        .expect("error running clippy")
        .stderr;
    (dir_path, String::from_utf8(stderr).unwrap())
}

fn has_problems(clippy_output: &String) -> bool {
    clippy_output.lines().count() > 1
}

fn proj_name_only(name_n_results: (String, String)) -> String {
    let mut name = name_n_results.0;
    let last_slash_ind = name.rfind("/").unwrap() + 1;
    name = name[last_slash_ind..].to_owned();
    name
}

fn print_results(need_cleanup: Vec<String>) {
    if need_cleanup.is_empty() {
        println!("All projects clean! Good job (:");
    } else {
        println!("Found clippy issues in the following projects: ");
        for proj in need_cleanup {
            println!("{}", proj);
        }
    }
}