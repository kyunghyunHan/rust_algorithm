use std::io::{self, BufRead, Write};
use std::collections::VecDeque;

pub fn main() {
    // 표준 입력 및 출력을 처리하기 위한 스트림 설정
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = io::BufWriter::new(stdout.lock());

    // 배열의 길이를 입력에서 읽어들임
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read input");
    let len: usize = input.trim().parse().expect("Invalid length");

    // 입력된 배열을 공백으로 분리하여 벡터로 변환
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read input");
    let string_array: Vec<&str> = input.split_whitespace().collect();
    let string_array: Vec<i32> = string_array.iter().map(|s| s.parse().expect("Invalid number")).collect();

    // 스택 및 결과 배열 초기화
    let mut stack = VecDeque::new();
    let mut nge = vec![-1; len];

    // 배열 순회하면서 오큰수 찾기
    for (i, &num) in string_array.iter().enumerate() {
        while let Some(&top) = stack.back() {
            if string_array[top] < num {
                // 오큰수 찾았을 때
                nge[stack.pop_back().unwrap()] = num;
            } else {
                break;
            }
        }
        stack.push_back(i);
    }

    // 오큰수를 출력
    for &result in nge.iter() {
        write!(writer, "{} ", result).expect("Failed to write output");
    }

    // 출력 버퍼 비우기
    writer.flush().expect("Failed to flush output");
}
