pub fn example() {
 

  
    p1();

    
}
static mut res: i32 = 0;
#[cfg(flase)]
fn p1(){
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
#[cfg(true)]
fn p1(){
    let mut pa: &str = "success";
    let mut pb: &str = "failure";

    println!("pa -> {}, pb ->{}",pa,pb);
    swap_ptr(&mut pa, &mut pb);
    println!("pa -> {}, pb ->{}",pa,pb);


}
fn swap_ptr<'a>(ppa: &mut &'a str, ppb: &mut &'a str) {
    let temp = *ppa;
    *ppa = *ppb;
    *ppb = temp;
}
fn sum(a: i32, b: i32) -> *mut i32 {
    static mut res: i32 = 0;
    unsafe{
        res = a+b;
    }
    &raw mut res
}
