use std::cmp::min;

use super::Status;

// just start with a range of 1 (0-1)
// treat the last element of the bound as upper bound and compare if element is greater or smaller
// if element is greater then bound then increase the bounded area by x2
// increase it untill element is lower then the bound
// and then perform a binary search in that bounded region
//
pub fn search(haystack: Vec<usize>, niddle: usize) -> Status {
    let mut bound = 1;
    while bound < haystack.len() && haystack[bound] < niddle {
        bound *= 2;
    }

    let lo = bound / 2;
    let hi = min(bound, haystack.len());
    return super::binary_search::recursive(haystack, niddle, lo, hi);
}

// #[test]
// #[should_panic]
fn _testing() {
    assert_eq!(search(vec![2, 4, 6, 8, 22, 32, 78], 32), Status::Found(5));
}
