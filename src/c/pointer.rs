pub fn example() {
 
 
//  { 
    
//     let mut to = [0u8; 100].as_mut_ptr();
//     let from: *const u8 = b"aa\0".as_ptr();
//     unsafe {
//         my_str_cpy(to, from);
//     }
// }

  let mut to :String = "".to_string();
  let from = "abc";
  my_str_cpy2(&mut to, from);
  println!("{}",to);



}
fn my_str_cpy2(to: &mut String, from: &str) {
    to.clear();

    for ch in from.chars() {
        to.push(ch);
    }
}

unsafe fn my_str_cpy(to: *mut u8, from: *const u8) -> *mut u8 {

    let mut dest = to;
    let mut src = from;

    // 널 문자('\0', 즉 0)를 만날 때까지 복사
    while *src != 0 {
        *dest = *src;
        dest = dest.add(1);
        src = src.add(1);
    }
    *dest = 0; // 마지막에 널 문자 추가

    to


}
fn swap(a:&mut i32 , b:&mut i32){

   let temp = *a;
   *a = *b;
   *b = temp;

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
#[cfg(false)]
fn p1(){
    let mut pa: &str = "success";
    let mut pb: &str = "failure";

    println!("pa -> {}, pb ->{}",pa,pb);
    swap_ptr(&mut pa, &mut pb);
    println!("pa -> {}, pb ->{}",pa,pb);


}
// fn swap_ptr<'a>(ppa: &mut &'a str, ppb: &mut &'a str) {
//     let temp = *ppa;
//     *ppa = *ppb;
//     *ppb = temp;
// }
// fn sum(a: i32, b: i32) -> *mut i32 {
//     static mut res: i32 = 0;
//     unsafe{
//         res = a+b;
//     }
//     &raw mut res
// }
