pub fn example() {
    // let mut a = 3;
    // let b = &mut a;
    // *b = 30;
    // println!("{}", a);
    // let mut resp: *mut i32;
    // let ( a, b) = (10, 20);
    // resp = sum( a,  b);
    // println!("{:?}",resp);
    // unsafe{
    //     println!("{}",*resp);
    // }

    let mut a = 10;
    let mut pi = &mut a;
    let mut ppi = &mut pi;
     **ppi = 20;
    println!("{}",a);

    let mut c = 10;
    let mut d :*mut i32 = &mut c;
    let mut e: *mut *mut i32  =&mut d;
    unsafe {
        **e = 100;
        println!("{}",*d);
    }
    

    
}
static mut res: i32 = 0;

fn sum(a: i32, b: i32) -> *mut i32 {
    static mut res: i32 = 0;
    unsafe{
        res = a+b;
    }
    &raw mut res
}
