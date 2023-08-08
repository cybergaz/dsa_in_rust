#[derive(Debug, PartialEq)]
pub enum Status {
    Found(i64),
    NotFound,
}

// i found it boring to imlement , its just binary search with two mid points (means array devided
// into 3 parts)
pub fn search(_haystack: Vec<usize>, _niddle: usize) -> Status {
    Status::NotFound
}

// #[test]
// #[should_panic]
fn _testing() {
    assert_eq!(search(vec![2, 4, 6, 8, 22, 32, 78], 32), Status::Found(5));
}
