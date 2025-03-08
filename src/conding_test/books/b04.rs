use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let n = i32::from_str_radix(&line, 2).unwrap(); // 2진수를 10진수로 변환
        writeln!(writer, "{}", n).unwrap();
    }
    writer.flush().unwrap();
}
