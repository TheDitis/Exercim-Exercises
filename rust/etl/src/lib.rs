use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut output = BTreeMap::new();
    for (&score, letters) in h.iter() {
        letters.iter().for_each(|c| {
            output.insert(c.to_ascii_lowercase(), score);
        })
    }
    output
}
