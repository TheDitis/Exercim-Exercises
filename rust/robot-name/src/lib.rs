use std::collections::HashSet;
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref ACTIVE_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self { Self::default() }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        Robot::mark_name_active(&self.name, false);
        self.name = Self::generate_name();
    }

    fn generate_name() -> String {
        let mut rng = thread_rng();
        let chars: String = (0..2).map(|_| char::from(rng.gen_range(65..=90))).collect();
        let nums: String = (0..3).map(|_| char::from(rng.gen_range(48..=57))).collect();
        let name = chars + nums.as_str();
        if Robot::name_is_active(&name) {
            Robot::generate_name()
        } else {
            Robot::mark_name_active(&name, true);
            name
        }
    }

    fn name_is_active(name: &String) -> bool {
        ACTIVE_NAMES.lock().unwrap().contains(name)
    }

    fn mark_name_active(name: &String, is_active: bool) {
        if is_active {
            ACTIVE_NAMES.lock().unwrap().insert(name.clone());
        } else {
            ACTIVE_NAMES.lock().unwrap().remove(name);
        }
    }
}

impl Default for Robot {
    fn default() -> Self {
        Self { name: Self::generate_name() }
    }
}
