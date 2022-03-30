
pub fn factors(num: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = Vec::new();
    let mut n = num;
    while n > 1 {
        for i in 2..=num {
            if !is_prime(i) { continue; }
            if n % i == 0 {
                divisors.push(i);
                n /= i;
                break;
            }
        }
    }
    divisors
}


pub fn is_prime(n: u64) -> bool {
    if n == 2 { return true }
    if n < 2 || n % 2 == 0 { return false }

    let mut i: u64 = 3;
    loop {
        if i.pow(2) > n { break true }
        if n % i == 0 { break false }
        i += 2;
    }
}