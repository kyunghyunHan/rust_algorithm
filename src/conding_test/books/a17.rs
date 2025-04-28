use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

/*

 Dungeon 2 (문제 A17)

 문제 요약:
 - 4개의 던전 'Dungeon 1'에서는 최단 시간에 몬스터를 퇴치하는 게임이 있습니다.
 - 이번에는 최단 시간으로 이동할 수 있는 방법을 찾는 프로그램을 작성합니다.
 - 던전에는 N개의 방이 있고, 각 방에는 Ai만큼의 몬스터와 Bi만큼의 보물이 있습니다.
 - 방 P1 -> 방 P2 -> ... -> 방 PK의 경로로 이동하는 경우를 찾아야 합니다.
 - 특히, P1=1, PK=N인 경우 중 최적의 경로를 찾아야 합니다.


 입력

5
2 4 1 3
5 3 7

출력
4
1 2 4 5
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
    let b = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();


}
