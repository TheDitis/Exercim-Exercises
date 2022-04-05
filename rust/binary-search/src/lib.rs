use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mid = array.len() / 2;
    // BASE CASE: No more sub-arrays to branch into
    if array.len() <= 1 {
        return match array.is_empty() {
            true => None,
            false => if array[0] == key { Some(mid) } else { None },
        }
    }
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Less => find(&array[..mid], key),
        Ordering::Greater => Some(mid + find(&array[mid..], key)?)
    }
}
