/* 계수 정렬 */
fn sort(arr: &mut Vec<usize>, n: usize, k: usize) -> Vec<usize> {
    let mut brr: Vec<usize> = vec![0; n + 1];
    let mut crr: Vec<usize> = vec![0; k + 1];

    // 빈도수 카운트
    for j in 0..=n {
        crr[arr[j]] += 1;
    }

    // 누적 빈도수 계산
    for i in 1..=k {
        crr[i] += crr[i - 1];
    }

    // 배열 정렬
    for j in (0..=n).rev() {
        brr[crr[arr[j]] - 1] = arr[j];
        crr[arr[j]] -= 1;
    }

    brr
}

pub fn example() {
    let mut arr: Vec<usize> = vec![16, 14, 10, 8, 7, 9, 3, 2, 4, 1];
    let n = arr.len() - 1;
    let k = *arr.iter().max().unwrap();

    let result = sort(&mut arr, n, k);
    println!("{:?}", result);
}