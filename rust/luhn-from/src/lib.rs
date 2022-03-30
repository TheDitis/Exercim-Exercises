pub struct Luhn {
    digits: Result<Vec<u32>, String>
}

impl Luhn {
    pub fn new(code: &str) -> Self {
        let digits = Luhn::format_code(code);
        Luhn { digits }
    }

    pub fn is_valid(&self) -> bool {
        match &self.digits {
            Ok(digits) => {
                let mut sum = 0;
                for (i, n) in digits.iter().rev().enumerate() {
                    if i % 2 != 0 {
                        sum += Self::sum_digits(n * 2);
                    } else {
                        sum += n;
                    }
                }
                digits.len() > 1 && sum % 10 == 0
            },
            Err(e) => {
                eprintln!("Luhn error: {}", e);
                false
            }
        }
    }

    /// Collects digits into a vec, throwing an error if invalid symbols are encountered
    pub fn format_code(code: &str) -> Result<Vec<u32>, String> {
        let mut digits: Vec<u32> = Vec::new();
        for char in code.chars() {
            match char.to_digit(10) {
                Some(n) => digits.push(n),
                None => {
                    if !char.is_whitespace() {
                        return Err("bad character type!".to_string());
                    }
                }
            }
        }
        if digits.len() <= 1 {
            return Err("Not enough digits".to_string());
        }
        Ok(digits)
    }

    fn sum_digits(n: u32) -> u32 {
        if n <= 9 {
            n
        } else {
            n.to_string().chars()
                .map(|c| c.to_digit(10).unwrap())
                .reduce(|acc, d| acc + d)
                .unwrap()
        }
    }
}


impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self::new(input.to_string().as_str())
    }
}
