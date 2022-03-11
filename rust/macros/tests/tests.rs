#[test]
fn test_empty() {
    let expected: HashMap<u32, u32> = HashMap::new();
    let computed: HashMap<u32, u32> = hashmap!();
    assert_eq!(computed, expected);
}

use std::collections::HashMap;
use macros::hashmap;

#[test]
fn test_single() {
    let mut expected = HashMap::new();
    expected.insert(1, "one");
    assert_eq!(hashmap!(1 => "one"), expected);
}