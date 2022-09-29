pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| {
        let char_num = u32::from(c);
        match char_num {
            97..=122 => char::from_u32(wrapped_in_range(char_num, key, 97, 122)).unwrap(),
            65..=90 => char::from_u32(wrapped_in_range(char_num, key, 65, 90)).unwrap(),
            _ => c
        }
    }).collect()
}

fn wrapped_in_range(i: u32, step: i8, floor: u32, ceil: u32) -> u32 {
    let wrap_size = ceil.wrapping_sub(floor) as i8 + 1;
    floor + ((i - floor) as i8 + step).rem_euclid(wrap_size) as u32
}