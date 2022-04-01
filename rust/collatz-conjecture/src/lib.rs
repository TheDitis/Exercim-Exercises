pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 { return None }
    let mut n = n;
    let mut i: u64 = 0;
    while n != 1 {
        i = i.checked_add(1)?;
        let rem = n % 2;
        if rem == 0 {
            n /= 2;
        } else {
            n = n.checked_mul(3)?.checked_add(1)?;
        };
    }
    Some(i)
}
