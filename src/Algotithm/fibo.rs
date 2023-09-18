use std::io;
use std::str;
pub fn main() {
    let mut input_one = String::new();
    io::stdin().read_line(&mut input_one).unwrap();
    let v1: Vec<u128> = input_one
        .split_whitespace()
        .map(|x| -> u128 { x.parse().unwrap() })
        .collect();
    let mut N = v1[0];
    println!("{}", fibo3(N))
}
//분할정복
fn fibo(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fibo(n - 2) + fibo(n - 1);
}
/*
다이나믹 프로그래밍은 이미 계산한 값을 메모리에 저장해 두었다가 나중에 다시 사용하는 기법
 */


/*
top-down
- 작은 문제들부터 
- 분할정복보다는 효율적이지만 재귀적인 함수호출로 인한 overhead
- 비효율
 */
fn fibo2(n:u128) -> u128 {
    let mut dp: Vec<u128> = vec![0; 100];  // 하위 문제 답을 저장할 메모이제이션 벡터

    if n <= 1 {
        return n
    }else{
         if dp[n as usize] > 0 {
          return dp[n as usize];
    }
    dp[n as usize] = fibo(n - 1) + fibo(n - 2);
    dp[n as usize]
    }

   

   
}
/*
Bottom-Up 
- 가장 작은 문제들부터 답을 구해가며 전체 문제의 답을 찾는방식
- 상향식
- 재귀호출을하지 않기 때문에 시간과 메모리 사용양을 줄일수 있다.

 */
fn fibo3(n: u128) -> u128 {
    let mut dp: Vec<u128> = vec![0; 200];  // 하위 문제 답을 저장할 메모이제이션 벡터
    dp[0] = 0;//가장작은 문제 이미 아는 값
    dp[1] = 1;//가장작은 문제 이미 아는 값
    for i in 2..=n {    // 2부터 시작해서 n까지 반복
        dp[i as usize] = dp[(i - 1) as usize] + dp[(i - 2) as usize];
    }
    dp[n as usize]
}