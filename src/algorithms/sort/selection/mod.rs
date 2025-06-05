/*
select sort
- 먼저 가장 작은 원소를 찾아 A[1]과 변경
*/

fn sort(arr: &mut Vec<i32>, n: usize) {
    for i in 0..n - 1 {
        //n-1번 반복
        let mut min_idx = i;
        for j in (i + 1)..n {
            //a[i..n-1]에서 최솟값 찾기
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }
        }
        // 최솟값과  a[i]의 위치교환
        arr.swap(i, min_idx);
    }
}
pub fn example() {
    let mut arr = vec![1, 3, 2, 7, 6];
    let n = arr.len();
    sort(&mut arr, n);
    println!("{:?}", arr);
}
