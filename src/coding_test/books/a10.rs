use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
/*

 Resort Hotel (문제 A10)

 문제 요약:
 - 리조트 호텔에는 1호실부터 N호실까지 N개의 방이 있습니다.
 - 호텔은 A_i일동안마다, 이 호텔에는 D일 동안 고객이 인원하면, d일차에는 L_d호실부터 R_d호실까지 방을 사용할 수 있습니다.
 - d=1, 2, ..., D에 대해, d일차에 사용할 수 있는 방 중에서 가장 큰 방은 몇 호실인지 출력하는 프로그램을 작성하시오.


  입력
7
1 2 4 4 2 3 1
2
3 5
4 6


 출력
3
5
*/
pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();

    reader.read_line(&mut input).unwrap();

    let n = input.parse::<usize>().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();

    let a = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let d = input.parse::<usize>().unwrap();

    for i in 0..d {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l = iter.next().unwrap().parse::<usize>().unwrap();
        let r = iter.next().unwrap().parse::<usize>().unwrap();
    }
}
