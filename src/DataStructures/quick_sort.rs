fn partition(a: &mut [usize], n: usize) -> usize {
    let mut left = 1;
    let mut right = n - 1;
//
    while left <= right {//피벗 a[0]
        while left < n && a[left] < a[0] {
            left += 1;
        }
        while right > 0 && a[right] >= a[0] {
            right -= 1;
        }
        if left < right {
            a.swap(left, right);
        } else {
            a.swap(0, right);
        }
    }
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
    let mut a = [1 ,5,8,2,3,4,6,9,7,10];
    let n = 10;
    quick_sort(&mut a, n);
    println!("{:?}", a);
}