use std::{
    cmp,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
};
/*


Dungeon 1 - Rust Solution
문제 요약:
- 어떤 던전에 1번부터 N까지의 번호가 붙어있는 N개의 방이 있습니다.
- 이 던전은 일방 통행이며, 들어올 때는 1개 또는 2개 입구 방으로 이동할 수 있습니다.
- 두 종류의 이동 시간은 다음과 같습니다:
  * 방 i→j에서 방 i로 옮기는 동류를 자는데 A_i분이 소요됩니다 (1 ≤ i ≤ N).
  * 방 i→j에서 방 j로 옮기는 동류를 자는데 B_j분이 소요됩니다 (1 ≤ i ≤ N).
- 결국 M(= N + 1)번 방까지 이동할 때 최소 몇 분이 걸리는지 구하는 프로그램을 작성합니다.


입력

5
2 4 1 3
5 3 7

출력
8
*/
pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut a = vec![0;100009];
    for i in 2..=n{
        input.clear();
        reader.read_line(&mut input).unwrap();
        a[i] = input.trim().parse::<i32>().unwrap();
    }
    input.clear();
    let mut b = vec![0;100009];
    for i in 3..=n{
        reader.read_line(&mut input).unwrap();
        b[i] = input.trim().parse::<i32>().unwrap();
    }

    let mut dp = vec![0; 100009];

    dp[1] = 0;
    dp[2] = 2;
    for i in 3..=n {
        dp[i] = (dp[i - 1] + a[i]).min(dp[i - 2] + b[i])
    }

    writeln!(writer, "{}", dp[n]).unwrap();
}
