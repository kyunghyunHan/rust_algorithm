use std::collections::HashMap;

/*Bottom -UP */
fn fibonacci(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    let mut dp = vec![0u64; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n]
}
/*Top-Down */

fn fibonacci2(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&val) = memo.get(&n) {
        return val;
    }
    let val = if n <= 1 {
        n
    } else {
        fibonacci2(n - 1, memo) + fibonacci2(n - 2, memo)
    };
    memo.insert(n, val);
    val
}
/* */
fn fibonacci3(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    b
}
pub fn example() {
    for i in 0..=20 {
        println!("fibonacci({}) = {}", i, fibonacci(i));
    }
}
