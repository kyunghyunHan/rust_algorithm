fn is_power_of_two(num: i32) -> bool {
    if num < 1 {
        return false;
    }

    let mut divided_num = num;
    while divided_num != 1 {
        if divided_num % 2 != 0 {
            return false;
        }
        divided_num /= 2;
    }
    return true;
}
pub fn example() {
    let num = 11;
    println!("{}",is_power_of_two(num));
}
