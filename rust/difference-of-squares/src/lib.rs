pub fn square_of_sum(n: u32) -> u32 {
    (1..=n).reduce(|acc, val| acc + val)
        .unwrap_or(0)
        .pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|v| v.pow(2))
        .reduce(|acc, val| acc + val)
        .unwrap_or(0)
}

pub fn difference(n: u32) -> u32 {
    (sum_of_squares(n) as i32 - square_of_sum(n) as i32).abs() as u32
}
