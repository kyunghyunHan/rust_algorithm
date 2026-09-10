//
const N: i32 = 5;
fn func05(l: i32) {
    if l > N {
        return;
    }
    println!("{}", l);
    func05(l + 1);
    println!("{}", l);
}
fn func04(l: i32) {
    if l > N {
        return;
    }

    func04(l + 1);
    println!("{}", l);
}
fn func03(l: i32) {
    if l == 0 {
        return;
    }
    func03(l / 2);
    println!("{}", l % 2);
}
fn func02() {}
fn print_binary(n: u32) {
    if n == 0 {
        return;
    }
    print_binary(n / 2);
    println!("{}", n % 2);
}
fn func01(l: i32) {
    println!("{}", l);
    func01(l + 1);
}
pub fn example() {
    func05(1);
}
