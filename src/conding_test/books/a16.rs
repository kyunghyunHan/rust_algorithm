use std::{
    cmp,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut a = vec![0; n + 1];

    let a_line = lines.next().unwrap().unwrap();
    let mut a_iter = a_line.trim().split_whitespace();
    for i in 2..=n {
        a[i] = a_iter.next().unwrap().parse().unwrap();
    }

    let mut b = vec![0; n + 1];

    let b_line = lines.next().unwrap().unwrap();
    let mut b_iter = b_line.trim().split_whitespace();
    for i in 3..=n {
        b[i] = b_iter.next().unwrap().parse().unwrap();
    }

    let mut dp = vec![0; n + 1];
    dp[1] = 0;
    dp[2] = a[2];

    for i in 3..=n {
        dp[i] = cmp::min(dp[i - 1] + a[i], dp[i - 2] + b[i]);
    }

    writeln!(writer, "{}", dp[n]).unwrap();
    writer.flush().unwrap();
}
