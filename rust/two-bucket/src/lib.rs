use std::cmp::min;
use std::mem;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Bucket {
    One,
    Two,
}
// impl Bucket {
//     fn other(&self) -> Self {
//         match self {
//             Self::One => Self::Two,
//             Self::Two => Self::One,
//         }
//     }
// }

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
    eprintln!("start_bucket = {:?}", start_bucket);
    let (mut smaller_, mut bigger_) = {
        let mut bucket1 = WaterBucket::new(capacity_1, Bucket::One);
        let mut bucket2 = WaterBucket::new(capacity_2, Bucket::Two);
        if matches!(*start_bucket, Bucket::One) { bucket1.fill() } else { bucket2.fill() }
        let mut sorted = [bucket1, bucket2];
        sorted.sort_by(|a, b| a.capacity.cmp(&b.capacity)); // why does reversed work? idk
        (*sorted.get(0).unwrap(), *sorted.get(1).unwrap())
    };
    let (smaller, bigger) = (&mut smaller_, &mut bigger_);
    let mut results = Vec::new();
    let mut start_state = (smaller.contains, bigger.contains);
    for l in 0..2 {
        eprintln!("\n\n\nl = {:?}", l);
        if l == 1 {
            smaller.contains = start_state.1;
            bigger.contains = start_state.0;
            mem::swap(&mut start_state.0, &mut start_state.1);
        }
        let mut i = 1;
        while smaller.contains != goal && bigger.contains != goal {
            eprintln!("\ni = {:?}", i);
            eprintln!("smaller = {:?}", smaller);
            eprintln!("bigger = {:?}", bigger);
            if smaller.capacity == goal || bigger.capacity == goal {
                if smaller.capacity == goal { smaller.fill() } else { bigger.fill() };
                println!("HERE");
            } else if smaller.is_full() {
                println!("emptying smaller");
                smaller.empty();
            } else if bigger.is_empty() {
                println!("filling bigger");
                bigger.fill();
            } else {
                bigger.pour_into(smaller);
            }

            i += 1;
            // End of loop check
            if let Some(res) = end_loop_check(smaller, bigger, i, goal, start_state, start_bucket) {
                println!("BREAKING");
                if res.is_some() { results.push(res.unwrap()) }
                break;
            }
        }

        eprintln!("results pre check = {:?}", results);
        if i == 1 {
            println!("HERE LINE 85");
            if let Some(Some(v)) = end_loop_check(smaller, bigger, i, goal, start_state, start_bucket) {
                results.push(v)
            }
        }
        eprintln!("results post check = {:?}", results);
        mem::swap(smaller, bigger);
    }
    if results.is_empty() {
        None
    } else {
        results.sort_by(|r1, r2| r1.moves.cmp(&r2.moves));
        eprintln!("results = {:?}", results);
        Some(results[0])
    }
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
        eprintln!("pouring {:?} into smaller", amount);
        target.contains += amount;
    }

    fn fill(&mut self) { self.contains = self.capacity }

    fn empty(&mut self) { self.contains = 0 }

    fn is_empty(&self) -> bool { self.contains == 0 }

    fn is_full(&self) -> bool { self.contains == self.capacity }
}


fn end_loop_check(
    smaller: &WaterBucket,
    bigger: &WaterBucket,
    i: u8,
    goal: u8,
    start_state: (u8, u8),
    start_bucket: &Bucket,
) -> Option<Option<BucketStats>> {
    if smaller.contains == goal {
        Some(Some(BucketStats {
            moves: i,
            goal_bucket: smaller.bucket_num,
            other_bucket: bigger.contains,
        }))
    } else if bigger.contains == goal {
        Some(Some(BucketStats {
            moves: i,
            goal_bucket: bigger.bucket_num,
            other_bucket: smaller.contains,
        }))
    } else if (matches!(smaller.bucket_num, start_bucket) && bigger.is_full() && smaller.is_empty())
        || (matches!(bigger.bucket_num, start_bucket) && smaller.is_full() && bigger.is_empty())
        || start_state == (smaller.contains, bigger.contains) {
        Some(None)
    } else {
        None
    }
}



