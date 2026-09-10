use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n: i32 = line.parse().unwrap();

        for x in (0..9).rev() {
            let wari = 1 << x;
            write!(writer, "{}", (n / wari) % 2).unwrap();
        }
        writeln!(writer).unwrap();
    }
    writer.flush().unwrap();
}
