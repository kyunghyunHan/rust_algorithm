use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<usize>().unwrap();

        if let Some(Ok(line)) = input.next() {
            let a = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let mut answer = false;

            for i in 0..n {
                for j in i + 1..n {
                    for k in j + 1..n {
                        if a[i] + a[j] + a[k] == 1000 {
                            answer = true;
                        }
                    }
                }
            }
            if answer == true {
                writeln!(writer, "{}", "Yes").unwrap();
            } else {
                writeln!(writer, "{}", "No").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
