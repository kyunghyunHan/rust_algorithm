/*
select sort
- 먼저 가장 작은 원소를 찾아 A[1]과 변경
*/

fn sort(arr: &mut Vec<i32>, n: usize) {
    for i in 0..n{
           // i번째 위치에 최소값을 넣기 위한 인덱스
           let mut min_idx = i;
           // 현재 i 이후로 배열에서 최소값을 찾는다.
           for j in (i + 1)..n {
               if arr[j] < arr[min_idx] {
                   min_idx = j;
               }
           }
           // i와 최소값을 교환한다.
           arr.swap(i, min_idx);
    }
}
pub fn example() {
   let mut arr = vec![5,2,4,6,1,3];
   let n = arr.len();
   sort(&mut arr, n);
   println!("{:?}", arr);
}
