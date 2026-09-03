pub fn sum_2d(a: *mut [i32; 4], r: usize, c: usize) -> i32 {
    let mut sum = 0;
    for i in 0..r {
        for j in 0..c {
            unsafe {
                sum += *(*a.add(i)).as_ptr().add(j);
            }
        }
    }

    sum
}

pub fn sum_1d(a: *mut i32, r: usize) -> i32 {
    let mut sum = 0;
    for i in 0..r {
        unsafe {
            sum += *a.add(i);
        }
    }
    sum
}

pub fn exchange(a: *mut *mut i32, b: *mut *mut i32) {
    unsafe {
        let temp = **a;

        **a = **b;
        **b = temp;
    }
}
