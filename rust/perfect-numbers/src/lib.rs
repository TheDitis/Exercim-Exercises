use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 { return None }
    Some(match aliquot_sum(num).cmp(&num) {
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Abundant,
        Ordering::Less => Classification::Deficient,
    })
}

pub fn aliquot_sum(num: u64) -> u64 {
    (1..=num/2).fold(0, |mut acc, i| {
        if num % i == 0 { acc += i }
        acc
    })
}