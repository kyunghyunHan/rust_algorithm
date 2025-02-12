use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let (n, x) = {
            let nx = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            (nx[0], nx[1])
        };

        if let Some(Ok(line)) = input.next() {
            let mut answer = false;
            let v = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            for i in 0..n {
                if v[i as usize] == x {
                    answer = true;
                }
            }

            if answer == true {
                writeln!(writer, "Yes").unwrap();
            } else {
                writeln!(writer, "No").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
