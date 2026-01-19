use std::arch::asm;
pub fn example() {
    let a = 1;
    let b = 2;
    println!("{}", add(a, b));
    println!("{}", add_asm(a, b));

}

#[inline(never)]
pub fn add_asm(a: i32, b: i32) -> i32 {
    let mut out: i32;
    unsafe {
        asm!(
            // "add {0:e}, {1:e}",
            // inout(reg) a => out, // out = a; out += b
            // in(reg) b,
            // options(pure, nomem, nostack),

            "add {out:w}, {a:w}, {b:w}",
            out = out(reg) out,
            a   = in(reg) a,
            b   = in(reg) b,
        );
    }
    out
}
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
