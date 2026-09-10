use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();
    if let Some(Ok(line)) = input.next() {
        let n = line.parse::<i32>().unwrap();

        writeln!(writer, "{}", n * n).unwrap();
    }
    writer.flush().unwrap();
}
