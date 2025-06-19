pub fn example() {
    let n = 20;
    println!("{:?}", prime_factors(n))
}

fn prime_factors(n: i32) -> Vec<i32> {
    let mut nn = n;

    let mut factors = vec![];

    for factor in 2..=nn.isqrt() {
        while nn % factor == 0 {
            nn /= factor;

            factors.push(factor);
        }
    }

    if nn != 1 {
        factors.push(nn);
    }
    return factors;
}
