use std::io;
use std::cmp::min;

pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut dp: Vec<Vec<String>> = vec![vec![String::new(); n + 1]; n + 1];
    
    for i in 1..=n {
        dp[i][0] = "1".to_string();
        dp[i][i] = "1".to_string();
    }

    let m = min(m, n - m);

    for i in 2..=n {
        for j in 1..=i / 2 {
            let s = big_num_add(&dp[i - 1][j - 1], &dp[i - 1][j]);
            dp[i][j] = s.clone();
            dp[i][i - j] = s;
        }
    }

    println!("{}", dp[n][m]);
}

fn big_num_add(s1: &str, s2: &str) -> String {
    let mut ret = String::new();
    let (s1, s2) = if s1.len() > s2.len() { (s2, s1) } else { (s1, s2) };
    let len1 = s1.len();
    let len2 = s2.len();
    let diff = len2 - len1;
    let mut carry = 0;
    let mut arr = vec![0; len2];

    for (i, c2) in s2.chars().rev().enumerate() {
        let i_rev = len2 - 1 - i;
        if i_rev >= diff {
            arr[i_rev] = s1.chars().nth(i_rev - diff).unwrap() as i32 + c2 as i32 - '0' as i32 + carry;
        } else {
            arr[i_rev] = c2 as i32 + carry;
        }

        if arr[i_rev] > '9' as i32 {
            arr[i_rev] -= 10;
            carry = 1;
        } else {
            carry = 0;
        }
    }

    if carry == 1 {
        ret.push('1');
    }

    for i in &arr {
        ret.push((*i as u8 + '0' as u8) as char);
    }

    ret
}
