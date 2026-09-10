use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = reader.lines();

    if let Some(Ok(line)) = input.next() {
        let d = line.parse::<usize>().unwrap();

        if let Some(Ok(line)) = input.next() {
            let n = line.parse::<usize>().unwrap();
            let mut l = vec![0i32; 100010];
            let mut r = vec![0i32; 100010];
            let mut b = vec![0i32; 100010];
            let mut answer = vec![0i32; 100010];
            for i in 1..=n {
                if let Some(Ok(line)) = input.next() {
                    let (ld, rd) = {
                        let lr = line
                            .trim()
                            .split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect::<Vec<i32>>();
                        (lr[0], lr[1])
                    };
                    l[i] = ld;
                    r[i] = rd;
                }
            }

            for i in 1..=n {
                b[l[i] as usize] += 1;
                b[(r[i] + 1) as usize] -= 1;
            }

            for j in 1..=d {
                answer[j] = answer[j - 1] + b[j];
            }
            for j in 1..=d {
                writeln!(writer, "{}", answer[j]).unwrap();
            }
        }
    }
    writer.flush().unwrap();
}
/*
8
5
2 3
3 6
5 7
3 7
1 5

*/
