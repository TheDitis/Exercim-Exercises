use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();
    for factor in factors {
        if *factor == 0 { continue; }
        for v in (*factor..limit).step_by(*factor as usize) {
            set.insert(v);
        }
    }
    set.iter().sum()
}
