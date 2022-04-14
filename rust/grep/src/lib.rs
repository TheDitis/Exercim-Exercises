use anyhow::Error;
use regex::Regex;
use std::fs;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Clone)]
pub struct Flags {
    line_numbers: bool,        // -n
    file_names_only: bool,     // -l
    case_sensitive: bool,      // -i
    invert_search: bool,       // -v
    match_whole_line: bool,    // -x
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        Flags {
            line_numbers: flags.contains(&"-n"),
            file_names_only: flags.contains(&"-l"),
            case_sensitive: !flags.contains(&"-i"),
            invert_search: flags.contains(&"-v"),
            match_whole_line: flags.contains(&"-x"),
        }
    }
}


struct Grep {
    pattern: Regex,
    flags: Flags,
    files: Vec<String>,
}

impl Grep {
    fn new(pattern: &str, flags: &Flags, files: &[&str]) -> Self {
        let pattern_str = if flags.case_sensitive {
            pattern.to_string()
        } else {
            pattern.to_lowercase()
        };
        Grep {
            pattern: Regex::new(pattern_str.as_str()).unwrap(),
            flags: flags.clone(),
            files: files.iter().map(|&s| s.to_string()).collect(),
        }
    }

    fn run(&mut self) -> Result<Vec<String>, Error> {
        let mut output: Vec<String> = Vec::new();
        for filename in &self.files {
            let matches = self.file_matches(filename)?;
            output.extend(matches);

        }
        println!("{:?}", output);
        Ok(output)
    }

    fn file_matches(&self, filename: &String) -> Result<Vec<String>, Error> {
        let file = String::from_utf8(fs::read(filename)?)?;
        let mut output: Vec<String> = Vec::new();
        for (i, line) in file.lines().enumerate() {
            if self.is_match(line) {
                if self.flags.file_names_only {
                    output.push(filename.to_string());
                    return Ok(output);
                } else if self.flags.line_numbers {
                    output.push(format!("{}:{}:{}", filename, i, line));
                } else {
                    output.push(format!("{}:{}", filename, line));
                }
            }
        }
        Ok(output)
    }

    fn is_match(&self, line: &str) -> bool {
        let line = if self.flags.case_sensitive { line.to_string() } else { line.to_lowercase() };
        let matches = if self.flags.match_whole_line {
            let line_match = self.pattern.find(line.as_str());
            if line_match.is_some() {
                println!("line_match: {}", line_match.unwrap().as_str());
            }
            line_match.is_some() && line_match.unwrap().as_str() == line
        } else {
            self.pattern.is_match(line.as_str())
        };
        if self.flags.invert_search { !matches } else { matches }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    Grep::new(pattern, flags, files).run()
}
