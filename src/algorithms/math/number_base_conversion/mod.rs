use std::io::{self, BufRead};

/// 10진수 → n진수 변환 함수
/// - num: 변환할 10진수 숫자
/// - base: 변환할 진법 (2~36)
/// - 반환값: 진법 변환된 문자열 (예: 2진수 "1010", 16진수 "FF")
fn to_base(mut num: u32, base: u32) -> String {
    // 사용할 숫자/문자 배열 (0~9, A~Z까지 총 36개)
    let digits = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // 최대 36진법 지원

    // 입력 숫자가 0이면 "0" 반환 (특수 케이스)
    if num == 0 {
        return "0".to_string();
    }

    // 변환 결과 저장용 벡터 (나중에 역순으로 뒤집어서 문자열로 변환)
    let mut res = Vec::new();

    // num이 0이 될 때까지 반복
    while num > 0 {
        // num을 base로 나눈 나머지를 문자로 변환해 저장
        res.push(digits[(num % base) as usize] as char);
        num /= base; // num을 base로 나눈 몫으로 업데이트 (자리 이동)
    }

    // 나머지를 역순으로 뒤집어 문자열로 만들어서 반환
    res.iter().rev().collect()
}

/// n진수 → 10진수 변환 함수
/// - s: 변환할 진법 문자열 (예: "FF", "1010")
/// - base: 입력 진법 (2~36)
/// - 반환값: 변환된 10진수 숫자
fn from_base(s: &str, base: u32) -> u32 {
    let digits = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut result = 0; // 누적 합을 저장할 변수

    // 문자열을 왼쪽부터 한 글자씩 처리
    for ch in s.chars() {
        // 해당 문자가 digits 배열에서 몇 번째 인덱스인지 찾음 (숫자값)
        let val = digits.iter().position(|&c| c as char == ch).unwrap() as u32;

        // 누적 결과에 base 곱하고 현재 자리 숫자 더함
        result = result * base + val;
    }

    result
}

/// Excel 열 제목 → 숫자 변환 함수
/// - s: Excel 열 제목 (예: "A", "AB", "ZY")
/// - 반환값: 해당 열 번호 (예: A=1, AB=28)
/// - Excel 특성상 A=1부터 시작 (0이 없음)
fn excel_to_number(s: &str) -> u32 {
    let mut result = 0;

    for ch in s.chars() {
        // 'A' 문자를 1로 맞춤 (A=1, B=2, ..., Z=26)
        let val = (ch as u8 - b'A' + 1) as u32;

        // 누적 결과에 26 곱하고 현재 문자 값 더함 (26진법)
        result = result * 26 + val;
    }

    result
}

/// 숫자 → Excel 열 제목 변환 함수
/// - num: 변환할 숫자 (1부터 시작)
/// - 반환값: Excel 열 제목 문자열 (예: 1→"A", 28→"AB")
/// - Excel 진법은 0이 없어서 변환 시 -1 보정 필요
fn number_to_excel(mut num: u32) -> String {
    let mut res = Vec::new();

    // 숫자가 0이 될 때까지 반복
    while num > 0 {
        num -= 1; // 0부터 시작하는 보정

        // 현재 자리 계산 후 문자로 변환해 저장
        res.push(((num % 26) as u8 + b'A') as char);

        // 자리 이동
        num /= 26;
    }

    // 뒤집어서 문자열로 반환
    res.iter().rev().collect()
}

fn main() {
    // 표준 입력 준비
    let stdin = io::stdin();

    // 입력 라인별로 처리 (한 줄만 읽음)
    for line in stdin.lock().lines() {
        let input = line.unwrap().trim().to_string();

        println!("[기본 진법 변환 예제]");

        let n10 = 35;
        println!("10진수 {} → 2진수: {}", n10, to_base(n10, 2));
        println!("10진수 {} → 16진수: {}", n10, to_base(n10, 16));

        println!("2진수 {} → 10진수: {}", "100011", from_base("100011", 2));
        println!("16진수 {} → 10진수: {}", "FF", from_base("FF", 16));

        println!("\n[Excel 특수 진법 변환 예제]");
        println!("'{}' → 숫자: {}", "AB", excel_to_number("AB"));
        println!("{} → Excel 제목: {}", 28, number_to_excel(28));

        break; // 한 번만 실행 후 종료
    }
}
