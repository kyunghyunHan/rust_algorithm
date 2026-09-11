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
/*
이제는 중복댄거없이
*/
fn test06(l: i32) {
    unsafe {
        if l == 3 {
            println!("{} {} {}", arr[0], arr[1], arr[2]);
            return;
        }
        for i in 1..=6 {
            if l > 0 && arr[l as usize - 1] >= i {
                continue;
            }
            arr[l as usize] = i;
            test06(l + 1);
        }
    }
}

fn test07(l: i32, mut sum: i32) {
    unsafe {
        if l == 3 {
            if sum == 10 {
                println!("{} {} {}", arr[0], arr[1], arr[2]);
            }
            return;
        }
        for i in 1..=6 {
            if l > 0 && arr[l as usize - 1] >= i {
                continue;
            }
            arr[l as usize] = i;
            test07(l + 1, sum + i);
        }
    }
}
pub fn example() {
    test07(0, 0);
}
