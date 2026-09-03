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
