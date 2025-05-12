fn sort(arr: &mut Vec<usize>, n: usize) {
    let mut brr = vec![0; n - 1];

    for i in 0..=n - 1 {
        brr[i] = 0;
    }
    for i in 1..=n {}
    for i in 0..=n - 1 {}
}
pub fn example() {
    let mut arr: Vec<usize> = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
    let n = arr.len() - 1;
    let k = *arr.iter().max().unwrap();

    // let result = sort(&mut arr, n, k);
    // println!("{:?}", result);
}
