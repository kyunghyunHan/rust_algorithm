fn quick_sort(a: &mut [i32], n: usize) {
    if n > 1 {
        let pivot = partition(a, n);

        // 피벗 기준 왼쪽 부분 정렬
        let left_slice = &mut a[0..pivot];
        quick_sort(left_slice, pivot);

        // 피벗 기준 오른쪽 부분 정렬
        if pivot + 1 < n {
            let right_slice = &mut a[pivot + 1..n];
            quick_sort(right_slice, n - pivot - 1);
        }
    }
}

fn partition(a: &mut [i32], n: usize) -> usize {
    let mut left = 1;
    let mut right = n - 1;

    while left < right {
        // 피벗(A[0])보다 큰 값의 위치를 왼쪽에서부터 찾음
        while left < n && a[left] < a[0] {
            left += 1;
        }

        // 피벗(A[0])보다 작은 값의 위치를 오른쪽에서부터 찾음
        while right > 0 && a[right] >= a[0] {
            right -= 1;
        }

        // 찾은 두 위치를 교환
        if left < right {
            a.swap(left, right);
        } else {
            // left와 right가 교차되면 피벗과 right 위치의 값을 교환
            a.swap(0, right);
        }
    }

    // 피벗의 최종 위치 반환
    right
}

pub fn example() {
    let mut a = [5, 6, 4, 10, 1, 9, 2, 8, 3, 7];
    let n = a.len();
    partition(&mut a, n);
    println!("{:?}", a);
    // let n = a.len();

    // println!("정렬 전: {:?}", a);

    // // 전체 퀵 정렬 적용
    // quick_sort(&mut a, n);
    // println!("정렬 후: {:?}", a);
}
