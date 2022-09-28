const M: i32 = 26;
const FLOOR: i32 = 97;
/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
/// E(x) = (ax + b) mod m
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 {
        return Err(AffineCipherError::NotCoprime(a))
    }
    let mut encoded: String = filter(plaintext).chars()
        .map(|c| {
            if let Some(x) = (c as u32).checked_sub(FLOOR as u32) {
                char::from_u32(((((a * x as i32) + b) % M) + FLOOR) as u32).unwrap()
            } else {
                c
            }
        })
        .collect();
    // insert spaces
    for i in (5..encoded.len()).step_by(5).rev() {
        encoded.insert(i, ' ');
    }
    Ok(encoded)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
/// D(y) = a^-1(y - b) mod m
pub fn decode(encoded: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 { return Err(AffineCipherError::NotCoprime(a)) }
    let decoded: String = filter(encoded).chars()
        .map(|c| {
            if c.is_ascii_digit() { return c; }
            let y = c as i32 - FLOOR;
            let char_num = (mod_inverse(a, M) * (y - b)).rem_euclid(M);
            char::from_u32((char_num + FLOOR) as u32).unwrap_or('@')
        })
        .collect();
    Ok(decoded)
}

fn filter(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else if a == b {
        a
    } else if a > b {
        gcd(a - b, b)
    } else {
        gcd(a, b - a)
    }
}

fn mod_inverse(a: i32, m: i32) -> i32 {
    for x in 1..m {
        if ((a % m) * (x % m)) % m == 1 {
            return x
        }
    }
    -1
}