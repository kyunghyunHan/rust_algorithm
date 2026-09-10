use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let (n, k) = {
            let nk = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            (nk[0], nk[1])
        };
        let mut cnt = 0;
        for x in 1..=n {
            for y in 1..=n {
                //이렇게 하면 느림
                // for z in 1..=n {
                //     if x + y + z == k {
                //         cnt += 1;
                //     }
                // }

                //2장의 카드를 결정하면 남은 1장의 카드도 결정된다를 이용
                let z = k - x - y;

                if z >= 1 && z <= n {
                    cnt += 1;
                }
            }
        }

        writeln!(writer, "{cnt}").unwrap();
    }
    writer.flush().unwrap();
}
