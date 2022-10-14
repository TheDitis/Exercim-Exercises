use std::cmp::min;
use std::mem;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut b = (WaterBucket::new(capacity_1, Bucket::One), WaterBucket::new(capacity_2, Bucket::Two));
    if matches!(*start_bucket, Bucket::One) { b.0.fill() } else { b.1.fill() }
    let mut results = Vec::new();
    let mut start_state = (b.0.contains, b.1.contains);
    for l in 0..2 {
        if l == 1 {
            b.0.contains = start_state.1;
            b.1.contains = start_state.0;
            mem::swap(&mut start_state.0, &mut start_state.1);
        }
        let mut i = 1;
        while b.0.contains != goal && b.1.contains != goal {
            if b.0.capacity == goal || b.1.capacity == goal {
                if b.0.capacity == goal { b.0.fill() } else { b.1.fill() };
            } else if b.0.is_full() {
                b.0.empty();
            } else if b.1.is_empty() {
                b.1.fill();
            } else {
                b.1.pour_into(&mut b.0);
            }
            i += 1;
            if let Some(res) = end_loop_check(&b.0, &b.1, i, goal, start_state, *start_bucket) {
                if let Some(v) = res { results.push(v) }
                break;
            }
        }

        if let Some(Some(v)) = end_loop_check(&b.0, &b.1, i, goal, start_state, *start_bucket) {
            results.push(v)
        }
        mem::swap(&mut b.0, &mut b.1);
    }
    if !results.is_empty() { results.into_iter().min_by(|a, b| a.moves.cmp(&b.moves)) } else { None }
}

#[derive(Clone, Copy, Debug)]
struct WaterBucket{
    capacity: u8,
    pub contains: u8,
    bucket_num: Bucket,
}

impl WaterBucket {
    pub fn new(capacity: u8, bucket_num: Bucket) -> Self {
        WaterBucket { capacity, contains: 0, bucket_num }
    }
    pub fn pour_into(&mut self, target: &mut Self) {
        let rem_capacity = target.capacity - target.contains;
        let amount = min(rem_capacity, self.contains);
        self.contains -= amount;
        target.contains += amount;
    }
    fn fill(&mut self) { self.contains = self.capacity }
    fn empty(&mut self) { self.contains = 0 }
    fn is_empty(&self) -> bool { self.contains == 0 }
    fn is_full(&self) -> bool { self.contains == self.capacity }
}

fn end_loop_check(
    b1: &WaterBucket,
    b2: &WaterBucket,
    i: u8,
    goal: u8,
    start_state: (u8, u8),
    start_bucket: Bucket,
) -> Option<Option<BucketStats>> {
    // if either one contains the goal amount, the result is a new BucketStats object
    if b1.contains == goal || b2.contains == goal {
        let (b1, b2) = if b1.contains == goal { (b1, b2) } else { (b2, b1) };
        Some(Some(BucketStats { moves: i, goal_bucket: b1.bucket_num, other_bucket: b2.contains }))
    }
    // if you've switched to the alternate starting point than the one given, result is None
    else if (b1.bucket_num == start_bucket && b2.is_full() && b1.is_empty())
        || (b2.bucket_num == start_bucket && b1.is_full() && b2.is_empty())
        || start_state == (b1.contains, b2.contains) {
        Some(None)
    }
    // No result in any other case
    else { None }
}



