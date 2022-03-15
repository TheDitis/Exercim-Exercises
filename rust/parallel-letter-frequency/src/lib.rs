use std::cmp::min;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let interval = min(worker_count, input.len()); // index interval between threads
    // Setup input and state for multi-thread access
    let map: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));
    let input: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(
        input.iter().map(|&s| s.to_string()).collect()
    ));
    let mut handles = vec![]; // to collect thread handles

    // from 0 to the number of threads we need (worker_count or input.len() if it's smaller)
    for j in 0..interval {
        // Get clones of our state and input variables for the next thread
        let state = Arc::clone(&map);
        let input = Arc::clone(&input);

        // Spawn a new thread to perform some of the calculation
        let handle = thread::spawn(move || {
            // request access to the input array and iterate indices in steps of 'interval'
            let input = input.lock().unwrap();
            for i in (j..input.len()).step_by(interval) {
                // get the str this thread will be running counts for and iterate over its chars
                let string = &input[i];
                for char in string.chars() {
                    // if it's a letter, request access, get/initialize its count, and increment it
                    if char.is_alphabetic() {
                        let mut map = state.lock().unwrap();
                        let letter = map.entry(char.to_ascii_lowercase()).or_insert(0);
                        *letter += 1;
                    }
                }
            }
        });
        // add this handle to the handles vec
        handles.push(handle);
    }
    // Make sure all threads have completed, then return the result
    for handle in handles {
        handle.join().unwrap();
    }
    Arc::try_unwrap(map).unwrap().into_inner().unwrap()
}
