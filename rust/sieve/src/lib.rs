use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut non_primes: HashSet<u64> = HashSet::new();
    let mut primes: Vec<u64> = Vec::new();

    for v in 2..=upper_bound {
        if non_primes.contains(&v) { continue }
        primes.push(v);
        for non_prime_v in (v*2..upper_bound+v).step_by(v as usize) {
            non_primes.insert(non_prime_v);
        }
    }

    primes
}
