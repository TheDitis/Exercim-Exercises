use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    validate_key(key)?;
    Some(s.chars().enumerate().map(|(i, c)| {
        let k = key.chars().nth(i % key.len()).unwrap();
        let [c_num, k_num] = [c, k].map(|x| x as u8 - 97);
        char::from(((c_num + k_num) % 26) + 97 )
    }).collect())
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    validate_key(key)?;
    Some(s.chars().enumerate().map(|(i, c)| {
        let k = key.chars().nth(i % key.len()).unwrap();
        let [c_num, k_num] = [c, k].map(|x| x as u8 - 97);
        char::from((((c_num as i32 - k_num as i32).rem_euclid(26)) + 97) as u8 )
    }).collect())
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = (0..=100).map(|_| {
        char::from(rand::thread_rng().gen_range(0..=25) + 97)
    }).collect();
    (key.clone(), encode(key.as_str(), s).unwrap())
}

fn validate_key(key: &str) -> Option<String> {
    if !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase()) {
        Some(key.to_string())
    } else {
        None
    }
}