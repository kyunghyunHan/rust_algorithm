use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
/*A 이상 B이하의 정수 중에서 100의 약수인 정수가 존재? */
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
        let mut answer = false;
        for i in n..=x {
            if 100 % i == 0 {
                answer = true;
            }
        }

        if answer == true {
            writeln!(writer, "Yes").unwrap();
        } else {
            writeln!(writer, "Yes").unwrap();
        }
    }
    writer.flush().unwrap();
}
