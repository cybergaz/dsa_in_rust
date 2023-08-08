pub fn sort(mut vec: Vec<usize>) -> Vec<usize> {
    for x in 0..vec.len() {
        for y in 0..vec.len() - 1 - x {
            if vec[y] > vec[y + 1] {
                let temp = vec[y];
                vec[y] = vec[y + 1];
                vec[y + 1] = temp;
            }
        }
    }
    vec
}

// #[test]
fn _testing() {
    use super::bubble::sort as bsort;
    assert_eq!(
        bsort(vec![2, 4, 6, 2, 1, 97, 23, 44]),
        vec![1, 2, 2, 4, 6, 23, 44, 97]
    )
}
