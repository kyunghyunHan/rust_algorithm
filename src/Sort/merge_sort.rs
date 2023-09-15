/*
합병정렬

분할정복의 방식을 사용해 데이터를 분할하고 분할한 집합을 정렬하며 합치는 알고리즘
- o(nlogn)
- 2개 그룹씩 합치며 오름차순으로 정렬
- 투포인터 개념을 사용하여 왼쪽 오른쪽 그룹을 병합,왼쪽 포인터와 오른쪽 포인터의 값을 비교하여 작은 값을 결과 배열에 추가하고 포인터를 오른쪽으로 1칸이동
*/

const MAX_SIZE: usize = 8;
//함병정렬
fn merge(list: &mut [i32], left: usize, mid: usize, right: usize) {
    let mut i = left;
    let mut j = mid + 1;
    let mut k = left;
    let mut sorted = vec![0; MAX_SIZE];

    while i <= mid && j <= right {
        if list[i] <= list[j] {
            sorted[k] = list[i];//비교해서 작은 값을 a[]로 이동
            i += 1;
        } else {
            sorted[k] = list[j];
            j += 1;
        }
        k += 1;
    }

    while i <= mid {
        sorted[k] = list[i];//남은 b[]의 데이터 
        //남은 c[]의데이터
        i += 1;
        k += 1;
    }

    while j <= right {
        sorted[k] = list[j];
        j += 1;
        k += 1;
    }

    for l in left..=right {
        list[l] = sorted[l];
    }
}

fn merge_sort(list: &mut [i32], left: usize, right: usize) {
    if left < right {
        let mid = (left + right) / 2;
        merge_sort(list, left, mid);//왼쪽 부분배열
        merge_sort(list, mid + 1, right);//오른쪽 부분 배열
        //합병과정
        merge(list, left, mid, right);
    }
}

pub fn main() {
    let mut list = [21, 10, 12, 20, 25, 13, 15, 22];
    let n = MAX_SIZE;
    merge_sort(&mut list, 0, n - 1);

    for i in 0..n {
        println!("{}", list[i]);
    }
}
//수행시간 O(nlogn)