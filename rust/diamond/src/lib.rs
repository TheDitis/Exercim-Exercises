pub fn get_diamond(c: char) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let c_num = c as u8 - 65;
    for (i, c) in (0..=c_num).map(|n| char::from(n + 65)).rev().enumerate() {
        let mut row: Vec<char> = (0..=c_num).map(|n| if n == i as u8 { c } else { ' ' }).collect();
        row.extend(flipped_n_reversed(&row, c_num));
        output.insert(0, row.iter().collect());
    }
    output.extend(flipped_n_reversed(&output, c_num));
    output
}

fn flipped_n_reversed<T: Clone>(v: &[T], up_to: u8) -> Vec<T> {
    v[0..up_to as usize].iter().cloned().rev().collect::<Vec<T>>()
}