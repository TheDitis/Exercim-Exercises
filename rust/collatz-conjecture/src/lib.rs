pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 { return None }
    let mut n = n;
    let mut i: u64 = 0;
    while n != 1 {
        match i.checked_add(1) {
            Some(v) => {i = v},
            None => return None,
        }
        let rem = n % 2;
        if rem == 0 {
            n = n / 2;
        } else {
            match n.checked_mul(3) {
                Some(v) => {
                    match v.checked_add(1) {
                        Some(v) => {n = v},
                        None => return None,
                    }
                },
                None => return None,
            }
        };
    }
    Some(i)
}
