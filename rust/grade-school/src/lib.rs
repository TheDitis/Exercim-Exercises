use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School { School::default() }

    pub fn add(&mut self, grade: u32, student: &str) {
        let entry = self.roster.entry(grade).or_insert(vec![]);
        let insert_pos = entry.iter()
            .position(|name| { student.cmp(name) == Ordering::Less })
            .unwrap_or(entry.len());
        entry.insert(insert_pos, student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.roster.keys().cloned().collect();
        grades.sort();
        grades
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster.get(&grade).unwrap_or(&vec![]).clone()
    }
}
