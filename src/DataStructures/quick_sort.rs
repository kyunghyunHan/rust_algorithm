fn partition(a: &mut [usize], n: usize) -> usize {
    let mut left = 1;
    let mut right = n - 1;
    let pivot = a[0]; // 피벗을 a[0]로 설정

    while left <= right {
        while left < n && a[left] <= pivot {
            left += 1;
        }
        println!("{:?}",a);
        while right > 0 && a[right] > pivot {
            right -= 1;
        }
        if left < right {
            a.swap(left, right);
        }
    }
    a.swap(0, right);
    right
}

fn quick_sort(a: &mut [usize], n: usize) {
    if n > 1 {
        let pivot = partition(a, n);
        quick_sort(&mut a[0..pivot], pivot);
        quick_sort(&mut a[pivot + 1..n], n - pivot - 1);
    }
}

pub fn main() {
    let mut a = [30, 35, 25, 55, 10, 50, 15, 45, 20 ,40];
    let n = 10;
    quick_sort(&mut a, n);
    println!("{:?}", a);
}
