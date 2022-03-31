pub fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    let n_digits = str_num.len() as u32;
    let sum = str_num.chars()
        .map(|digit| digit.to_digit(10).unwrap().pow(n_digits))
        .reduce(|a, b| a + b).unwrap();
    sum == num
}
