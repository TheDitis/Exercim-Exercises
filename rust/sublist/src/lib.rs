use std::fmt::Display;
use crate::Comparison::{Equal, Sublist, Superlist, Unequal};

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Display>(list1: &[T], list2: &[T]) -> Comparison {
    match (is_sublist(list1, list2), is_sublist(list2, list1)) {
        (true, true) => Equal,
        (true, false) => Sublist,
        (false, true) => Superlist,
        (false, false) => Unequal,
    }
}

fn is_sublist<T: PartialEq + Display>(list1: &[T], list2: &[T]) -> bool {
    sublist_inds(list1, list2).is_some()
}

pub fn sublist_inds<T: PartialEq + Display>(list1: &[T], list2: &[T]) -> Option<(i32, i32)> {
    if list1.is_empty() { return  Some((0, 0)) }  // case: empty list1 is a sublist

    for j in 0..list2.len() {
        if j + list1.len() > list2.len()  { break }
        let sub2 = &list2[j..j + list1.len()];
        if sub2 == list1 {
            return Some((j as i32, (j + list1.len() - 1) as i32));
        }
    }
    None
}