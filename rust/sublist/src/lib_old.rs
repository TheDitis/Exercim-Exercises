se std::fmt;
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

pub fn sublist_inds<T: PartialEq + Display>(list1: &[T], list2: &[T]) -> Option<(i32, i32)> {
    if list2.len() < list1.len() { return None }
    if list1.len() == 0 { return  Some((0, 0)) }

    let mut inds: (i32, i32) = (-1, -1);
    // for each item in the 'larger' list:
    for j in 0..list2.len() {
        // see if list2[j : j + list1.len()] == list1
        let sub2 = list2.slice(j, j + list1.len());
        if sub2 == list1 {

        }
        for (i, v1) in list1.iter().enumerate() {
            // if i + j is out of bounds or current items don't match, reset inds and break
            if j + i >= list2.len() || list2[j + i] != *v1 {
                inds = (-1, -1);
                break;
            }
            // otherwise, if just starting iteration over potential sublist (list1), set inds.0
            if i == 0 {
                inds.0 = j as i32;
            }
            inds.1 = (j + i) as i32; // set inds.1 to current j + 1

        }
        // if a full match has already been found, exit the loop!
        if inds.1 - inds.0 == (list1.len() as i32 - 1) { break; }
        // otherwise, reset inds
        inds = (-1, -1)
    }

    // Return None if inds are -1, Some(inds) otherwise
    match inds {
        (-1, -1) => None,
        _ => Some(inds),
    }
}

fn is_sublist<T: PartialEq + Display>(list1: &[T], list2: &[T]) -> bool {
    sublist_inds(list1, list2).is_some()
}