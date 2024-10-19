fn bubble_sort(arr: &mut Vec<usize>, n: usize) {
    for i in 1..n {
        for j in (i..n).rev() {
            if arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
            }
        }
    }
}

pub fn example() {
    let mut arr = vec![31, 41, 59, 26, 41, 58];
    let n = arr.len();
    bubble_sort(&mut arr, n);
    println!("{:?}", arr);
}
