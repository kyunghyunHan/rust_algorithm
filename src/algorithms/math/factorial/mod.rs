fn factorial(num: i64) -> i64 {
    let mut result = 1;
    for i in 2..=num {
        result *= i;
    }
    result
}

pub fn example() {
    println!("{}", factorial(20));
}
