use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    let mut a = [0; 100009];
    let mut b = [0; 100009];
    let mut dp = [0; 100009];

    if let Some(Ok(line)) = input.next() {
        let n: usize = line.parse().unwrap();

        if let Some(Ok(line)) = input.next() {
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            for (i, &val) in parts.iter().enumerate() {
                a[i + 2] = val;
            }
        }

        if let Some(Ok(line)) = input.next() {
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            for (i, &val) in parts.iter().enumerate() {
                b[i + 3] = val;
            }
        }

        dp[1] = 0;
        dp[2] = a[2];

        for i in 3..=n {
            dp[i] = std::cmp::min(dp[i - 1] + a[i], dp[i - 2] + b[i]);
        }

        writeln!(writer, "{}", dp[n]).unwrap();
    }
    writer.flush().unwrap();
}
