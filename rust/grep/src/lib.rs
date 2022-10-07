use anyhow::Error;
use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    line_numbers: bool,        // -n
    file_names_only: bool,     // -l
    case_sensitive: bool,      // -i
    invert_search: bool,       // -v
    match_whole_line: bool,    // -x
}
//
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
    pattern: String,
    flags: Flags,
    files: Vec<String>,
}
impl Grep {
    pub fn new(pattern: &str, flags: &Flags, files: &[&str]) -> Self {
        Grep {
            pattern: pattern.to_string(),
            flags: flags.clone(),
            files: files.iter().cloned().map(String::from).collect()
        }
    }

    pub fn run(&self) -> Result<Vec<String>, Error> {
        let mut output = Vec::new();
        let pattern = Regex::new(
            format!("{}({})", if !self.flags.case_sensitive { "(?i)" } else { "" }, self.pattern).as_str()
        )?;
        for file_name in &self.files {
            let content = String::from_utf8(fs::read(file_name)?)?;
            for (line_num, line) in content.split('\n').enumerate() {
                if !line.is_empty() && self.matches_line(line, &pattern) {
                    let mut entry = vec![];
                    if self.files.len() > 1 || self.flags.file_names_only {
                        entry.push(file_name.clone());
                    }
                    if !self.flags.file_names_only {
                        if self.flags.line_numbers { entry.push((line_num + 1).to_string()) };
                        entry.push(line.to_string())
                    };
                    output.push(entry.join(":"));
                    if self.flags.file_names_only { break }
                }
            }
        }
        Ok(output)
    }

    fn matches_line(&self, line: &str, pattern: &Regex) -> bool {
        let match_line = if self.flags.match_whole_line { Grep::whole_line_matches } else { Grep::line_contains_match };  // -x
        let match_ = match_line(line, pattern);
        if self.flags.invert_search { !match_ } else { match_ }
    }

    fn line_contains_match(line: &str, pattern: &Regex) -> bool {
        pattern.find(line).is_some()
    }
    fn whole_line_matches(line: &str, pattern: &Regex) -> bool {
        if let Some(match_) = pattern.find(line.strip_suffix('\n').unwrap_or(line)) {
            match_.start() == 0 && match_.end() == line.len()
        } else { false }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    Grep::new(pattern, flags, files).run()
}
