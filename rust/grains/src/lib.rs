extern crate core;

pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("Square must be between 1 and 64");
    }
    let mut n = 1;
    for _ in 1..s {
        n *= 2;
    }
    n
}

pub fn total() -> u64 {
    let mut n = 0;
    for i in 1..=64 { n += square(i); }
    n
}