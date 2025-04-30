pub fn greedy_coin_change(coins: &[i32], amount: i32) -> i32 {
    let mut remaining = amount;
    let mut count = 0;

    for &coin in coins {
        let used = remaining / coin;
        count += used;
        remaining -= used * coin;
    }

    if remaining > 0 {
        -1
    } else {
        count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy_coin_change() {
        assert_eq!(greedy_coin_change(&[500, 100, 50, 10, 5, 1], 4200), 10); // 500*8 + 100*2 = 4200
        assert_eq!(greedy_coin_change(&[500, 100, 50, 10, 5, 1], 123), 6); // 100 + 10*2 + 1*3 = 123

    }
}
