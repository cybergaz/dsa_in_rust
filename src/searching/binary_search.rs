use super::Status;

pub fn search(haystack: Vec<usize>, niddle: usize) -> Status {
    let mut lo = 0;
    let mut hi = haystack.len();
    // println!("{hi}");

    while lo < hi {
        let mid = (lo + hi) / 2;
        let val = haystack[mid];

        if val == niddle {
            return Status::Found(mid as i64);
        } else if val > niddle {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    Status::NotFound
}

pub fn recursive(haystack: Vec<usize>, niddle: usize, lo: usize, hi: usize) -> Status {
    if lo > hi {
        return Status::NotFound;
    }
    let mid = (lo + hi) / 2;
    if haystack[mid] == niddle {
        return Status::Found(mid as i64);
    } else if haystack[mid] > niddle {
        return recursive(haystack, niddle, lo, mid);
    } else {
        return recursive(haystack, niddle, mid + 1, hi);
    }
}

// #[test]
// #[should_panic]
fn _testing() {
    assert_eq!(search(vec![2, 4, 6, 8, 22, 32, 78], 78), Status::Found(6));
    assert_eq!(
        recursive(vec![2, 4, 6, 8, 22, 32, 78], 32, 0, 7),
        Status::Found(5),
    );
}
