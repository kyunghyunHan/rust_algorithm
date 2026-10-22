use std::alloc::{alloc, dealloc, Layout};

/*
RAII Pattern


*/
pub fn main() {
    unsafe {
        let fp = [1, 2];
        let mut fpp: *const i32 = fp.as_ptr();
    
        let layout = Layout::array::<fn(i32, i32) -> i32>(5).unwrap();

        let fpp = alloc(layout) as *mut fn(i32, i32) -> i32;

   
        dealloc(fpp as *mut u8, layout);
    }
}
