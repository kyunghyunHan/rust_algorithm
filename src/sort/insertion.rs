fn sort(arr: &mut Vec<i32>, n: usize) {
    for i in 1..n {
        let key = arr[i];
        //a[i]를 정렬된 배열 A[1:i-1]에 삽입한다
        let mut j = i - 1;

        while j > 0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j -= 1;
        }
        arr[j + 1] = key;
    }
}

pub fn example() {
    let mut arr = vec![31, 41, 59, 26, 41, 58];
    let n = arr.len();
    sort(&mut arr, n);
    println!("{:?}", arr);
}
