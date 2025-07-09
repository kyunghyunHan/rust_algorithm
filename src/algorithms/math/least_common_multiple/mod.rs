use super::euclidean::euclidean_algorithm;
fn least_common_multiple(a: i64, b: i64) -> i64 {
    if (a == 0) || (b == 0) {
        return 0;
    } else {
        a * b / euclidean_algorithm(a, b)
    }
}

pub fn example() {

    
}
