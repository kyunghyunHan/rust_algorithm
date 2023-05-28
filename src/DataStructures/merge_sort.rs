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