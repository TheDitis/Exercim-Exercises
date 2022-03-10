pub fn is_armstrong_number(num: u32) -> bool {
    let str_num = num.to_string();
    let n_digits = str_num.len();
    let pow_digits: Vec<u32> = str_num.chars()
        .map(|digit| {
            println!("digit: {:?}", digit);
            digit.to_digit(10).unwrap().pow(n_digits as u32)
        }).collect();
    let sum = pow_digits.into_iter().reduce(|a, b| a + b).unwrap();
    println!("num: {:?}   sum: {:?}", num, sum);
    sum == num
}
