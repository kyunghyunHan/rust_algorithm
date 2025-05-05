pub fn euclidean_algorithm(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();

    if b == 0 {
        a
    } else {
        euclidean_algorithm(b, a % b)
    }
}

pub fn example() {
    let a = 252;
    let b = 105;

    let result = euclidean_algorithm(a, b);
    println!("GCD of {} and {} is {}", a, b, result);
}
