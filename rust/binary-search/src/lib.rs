
pub fn find(array: &[i32],  key: i32) -> Option<u32> {
    let mid = array.len() / 2;
    // BASE CASE: No more sub-arrays to branch into
    if array.len() <= 1 {
        return match array.is_empty() {
            true => None,
            false => if array[0] == key { Some(mid as u32) } else { None },
        }
    }
    // If key is lower, check left subarray
    if key < array[mid] {
        let left = &array[..mid];
        let sub_ind = find(left, key)?;
        Some((mid - (left.len() - sub_ind as usize)) as u32)
    }
    // if key is greater, check right subarray
    else {
        let sub_ind = find(&array[mid..], key)?;
        Some(mid as u32 + sub_ind)
    }
}
