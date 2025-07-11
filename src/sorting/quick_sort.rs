pub fn qwik<T: Copy + std::cmp::PartialOrd>(heystack: &mut [T]) {
    if heystack.len() <= 1 {
        return;
    }
    let p = partition(heystack);
    // println!(
    //     "array after partition : {:?} and partition index : {} ",
    //     heystack, p
    // );
    qwik(&mut heystack[..p]);
    qwik(&mut heystack[p + 1..]);
}

fn partition<T: Copy + std::cmp::PartialOrd>(heystack: &mut [T]) -> usize {
    let length = heystack.len();
    let pivot = heystack[length - 1];
    let mut boundary = 0;

    for i in 0..length - 1 {
        if heystack[i] <= pivot {
            heystack.swap(i, boundary);
            boundary += 1;
        }
    }
    heystack.swap(length - 1, boundary);

    boundary
}

#[ignore]
fn _testing() {
    let mut vector = vec![4, 545, 34, 2, 33, 211, 3, 9, 1, 15, 22, 16];
    // let mut vector = vec![4, 545, 3, 16];
    //                   [ 4 , 3 , 545 , 16]
    //                   [ 4 , 3 , 16 , 545 ]
    println!("{:#?}", vector);
    qwik(&mut vector);
    println!("{:#?}", vector);
}
