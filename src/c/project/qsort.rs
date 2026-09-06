use std::os::raw::c_void;
use std::{ptr, ptr::copy};
pub fn example() {
    let mut arr = [[3, 5, 1], [1, 4, 3], [7, 1, 5], [3, 2, 2], [5, 3, 4]];
    let base = arr.as_mut_ptr();
    let len = arr.len();
    let size = std::mem::size_of::<[i32; 3]>();
    ssort(base as *mut c_void, len, size, comp);
    print_arr(base, len);

    sssort(base, len);
    print_arr(base, len);
}

fn print_arr(arr: *const [i32; 3], rows: usize) {
    unsafe {
        for i in 0..rows {
            for j in 0..3 {
                print!("{} ", (*arr.add(i))[j]);
            }
            println!();
        }
    }
}

fn comp(a: *const c_void, b: *const c_void) -> i32 {
    unsafe {
        let ca = *(a as *const i32);
        let cb = *(b as *const i32);

        return (ca > cb) as i32 - (ca < cb) as i32;
    }
}
fn swap(a: *mut c_void, b: *mut c_void, size: usize) {
    let mut temp = vec![0u8; size];
    unsafe {
        ptr::copy(a as *const u8, temp.as_mut_ptr(), size);
        ptr::copy(b as *mut u8, a as *mut u8, size);
        ptr::copy(temp.as_ptr(), b as *mut u8, size);

        drop(temp);
    }
}

fn ssort(
    base: *mut c_void,
    len: usize,
    size: usize,
    comp: fn(*const c_void, *const c_void) -> i32,
) {
    unsafe {
        let arr = base as *mut u8;
        for i in 0..len - 1 {
            for j in i + 1..len {
                let a = arr.add(i * size) as *mut c_void;
                let b = arr.add(j * size) as *mut c_void;
                if comp(a, b) > 0 {
                    swap(a, b, size);
                }
            }
        }
    }
}

fn sssort(arr: *mut [i32; 3], rows: usize) {
    unsafe {
        for i in 0..rows {
            ssort(
                arr.add(i) as *mut c_void,
                3,
                std::mem::size_of::<i32>(),
                comp,
            );
        }
        ssort(
            arr as *mut c_void,
            rows,
            std::mem::size_of::<[i32; 3]>(),
            comp,
        );
    }
}
