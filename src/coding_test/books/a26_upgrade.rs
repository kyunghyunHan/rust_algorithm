use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut lines = reader.lines();

    let q: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x = Vec::with_capacity(q);

    for _ in 0..q {
        if let Some(Ok(line)) = lines.next() {
            x.push(line.trim().parse::<usize>().unwrap());
        }
    }

    let n = 300000;
    let mut deleted = vec![false; n + 1];

    // 에라토스테네스의 체
    for i in 2..=((n as f64).sqrt() as usize) {
        if !deleted[i] {
            for j in (i * 2..=n).step_by(i) {
                deleted[j] = true;
            }
        }
    }

    for &num in &x {
        if num <= n && !deleted[num] {
            writeln!(writer, "Yes").unwrap();
        } else {
            writeln!(writer, "No").unwrap();
        }
    }
    writer.flush().unwrap();
}