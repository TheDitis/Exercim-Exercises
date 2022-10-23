
/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let mut rev = 0;
        let mut val = value;
        while val > 0 {
            let last_digit = val % 10;
            rev = rev * 10 + last_digit;
            val /= 10;
        }
        if value == rev { Some(Palindrome(value)) } else { None }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_pal = None;
    let mut max_pal = None;

    for v1 in min..=max {
        for v2 in v1..=max {
            if let Some(palindrome) = Palindrome::new(v1 * v2) {
                if min_pal.is_none() || palindrome < min_pal.unwrap() { min_pal = Some(palindrome) }
                if max_pal.is_none() || palindrome > max_pal.unwrap() { max_pal = Some(palindrome) }
            }
        }
    }
    if min_pal.is_none() || max_pal.is_none() { return None }
    Some((min_pal.unwrap(), max_pal.unwrap()))
}
