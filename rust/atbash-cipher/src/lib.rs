const FIRST: [char; 13] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];
const LAST: [char; 13] = ['z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n'];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded = atbash(plain);
    space_groupings(encoded, 5)
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    atbash(cipher)
}

fn filter(raw: &str) -> String {
    raw.chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_digit())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn atbash(input: &str) -> String {
    filter(input).chars()
        .map(|c| {
            let char_num = u32::from(c);
            match char_num {
                97..=109 => LAST[(char_num - 97) as usize],
                110..=122 => FIRST[25 - (char_num - 97) as usize],
                _ => char::from_u32(char_num).unwrap()
            }
        })
        .collect()
}

fn space_groupings(input: String, size: usize) -> String {
    input.chars().enumerate()
        .flat_map(|(i, c)| if i >= size && i % size == 0 { vec![' ', c] } else { vec![c] })
        .collect()
}