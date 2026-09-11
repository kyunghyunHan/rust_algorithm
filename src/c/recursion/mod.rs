const N: i32 = 5;
static mut arr: [i32; 3] = [0; 3];
/*
1 2 3 4 5 출력

*/
fn test01(l: i32) {
    if l > N {
        return;
    }
    print!("{} ", l);
    test01(l + 1);
}
/*1 2 3 4 5 5 4 3 2 1 */
fn test02(l: i32) {
    if l > N {
        return;
    }
    print!("{} ", l);
    test02(l + 1);
    print!("{} ", l);
}
/*1 2 3 4 5 4 3 2 1 */
fn test03(l: i32) {
    if l > N - 1 {
        print!("{} ", l);
        return;
    }
    print!("{} ", l);
    test03(l + 1);
    print!("{} ", l);
}
/*1 2 3 4 5 1 2 3 4*/
fn test04(l: i32) {
    if l > N {
        return;
    }
    print!("{} ", l);
    test04(l + 1);
    print!("{} ", N - l + 1);
}
/*
1 1 1
1 1 2
1 1 3
1 2 1
*/
fn test05(l: i32) {
    unsafe {
        if l == 3 {
            println!("{} {} {}", arr[0], arr[1], arr[2]);
            return;
        }

        for i in 1..=3 {
            arr[l as usize] = i;
            test05(l + 1);
        }
    }
}
pub fn example() {
    test05(0);
}
