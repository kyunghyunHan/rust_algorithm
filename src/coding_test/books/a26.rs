/*prime check */
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn is_prime(x: i32) -> bool {
    for i in 2..x {
        if x % i == 0 {
            return false;
        }
    }
    true
}
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let q = line.trim().parse::<i32>().unwrap();
        let mut x = [0;100009];
        for i in 1..=q {
            if let Some(Ok(line)) = input.next() {
                x[i as usize] = line.parse::<i32>().unwrap();
            }
        }
        for i in 1..=q {
            let answer = is_prime(x[i as usize]);
            if answer == true {
                writeln!(writer, "Yes").unwrap();
            } else {
                writeln!(writer, "No").unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
