fn search(arr: &mut [i32], n: usize, x: i32) -> usize {
    let mut i = 0;
    while i < n && arr[i] != x {
        i = i + 1;
    }
    return i;
}

pub fn example() {
    let mut arr = vec![60, 20, 50];
    let n = arr.len();
    let x = 50;
    println!("{}", search(&mut arr, n, x));
}
