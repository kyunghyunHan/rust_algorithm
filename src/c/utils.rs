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

pub fn print_1d_array(ary: *const i32, n: i32) {
    for i in 1..=n {
        unsafe {
            print!("{:?} ", *ary.add(i as usize));
        }
    }
    print!("\n");
}

pub fn print_var_array(pary: *const *const i32, n: i32) {
    for i in 0..n {
        unsafe {
            print_1d_array(*pary.add(i as usize), **(pary.add(i as usize)));
        }
    }
}
