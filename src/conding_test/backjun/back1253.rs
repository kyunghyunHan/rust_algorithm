use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if let Some(Ok(line)) = input.next() {
            let mut v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            v.sort_by(|a, b| a.cmp(b));
            let mut result = 0;
            for k in 0..n {
                let mut find = v[k];
                let mut i = 0;
                let mut j = n - 1;

                while i < j {
                    if (v[i] + v[j]) == find {
                        if (i != k && j != k) {
                            result += 1;
                            break;
                        } else if i == k {
                            i += 1;
                        } else if j == k {
                            j -= 1;
                        }
                    } else if (v[i] + v[j]) < find {
                        i += 1;
                    } else {
                        j -= 1;
                    }
                }
            }
            writeln!(writer, "{}", result).unwrap();
        }
    }
    writer.flush().unwrap();
}
