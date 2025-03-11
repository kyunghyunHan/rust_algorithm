/*바이너리 서치
작은 값부터 순서대로 배열되어 있는 요소수가 A개인 배열 요소 X는 배열 A의 몇번쨰에 있는지
*/
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

fn search(x: i32, n: i32, a: Vec<i32>) -> i32 {
    let (mut l, mut r) = (1, n);
    while l <= r {
        let m = (l + r) / 2;
        if x < a[m as usize] {
            r = m - 1;
        }
        if x == a[m as usize] {
            return m;
        }

        if x > a[m as usize] {
            l = m + 1;
        }
    }
    -1
}

fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());
    let mut input = reader.lines();
    let mut a = [0; 100009];
    if let Some(Ok(line)) = input.next() {
        let (n, x) = {
            let nx = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            (nx[0], nx[1])
        };

        if let Some(Ok(line)) = input.next() {
            let a = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>();
            let answer = search(x, n, a);
            writeln!(writer,"{}",answer+1).unwrap();
        }
    }
}
