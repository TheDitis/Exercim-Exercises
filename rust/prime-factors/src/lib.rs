const PRIME_DIVISORS: [u64; 5] = [2, 3, 5, 7, 11];

pub fn factors(num: u64) -> Vec<u64> {
    let mut divisors: Vec<u64> = Vec::new();
    let mut n = num;
    while n > 1 {
        for div in PRIME_DIVISORS {
            if n % div == 0 {
                divisors.push(div);
                n /= div;
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