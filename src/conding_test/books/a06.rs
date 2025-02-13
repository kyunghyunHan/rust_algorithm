use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let (n, q) = {
            let nq = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            (nq[0], nq[1])
        };

        if let Some(Ok(line)) = input.next() {
            let a = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>();
            let mut larr = [0; 100009];
            let mut rarr = [0; 100009];
            let mut sarr = [0; 100009];
            for j in 1..=q {
                if let Some(Ok(line)) = input.next() {
                    let (l, r) = {
                        let lr = line
                            .trim()
                            .split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<usize>>();
                        (lr[0], lr[1])
                    };
                    larr[j] = l;
                    rarr[j] = r;
                }
            }
            sarr[0] = 0;
            for i in 1..=n {
                sarr[i] = sarr[i - 1] + a[i-1]
            }

            for j in 1..=q {
                writeln!(writer, "{}", sarr[rarr[j]] - sarr[larr[j] - 1]).unwrap();
            }
        }
    }
}
