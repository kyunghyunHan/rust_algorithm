use std::{
    f64::consts::PI,
    io::{stdin, stdout, BufRead, BufReader, BufWriter, Write},
 };
 
 // 원점(0,0)으로부터의 거리 계산 함수
 fn distance(x: f64, y: f64) -> f64 {
    let distance = (x.powi(2) + y.powi(2)).sqrt();  // 피타고라스 정리 사용
    distance
 }
 
 // 벡터의 크기(길이) 계산 함수
 fn vector_size(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()  // distance와 같은 공식 사용
 }
 
 // 두 벡터의 내적 계산 함수
 fn inner_fn(x: f64, y: f64, x1: f64, y1: f64) -> f64 {
    (x * x1) + (y * y1)  // 내적 공식: (x*x1 + y*y1)
 }
 
 fn main() {
    // 입력값 설정
    let x = -1;  // 캐릭터의 x좌표
    let y = 2;   // 캐릭터의 y좌표
    let r = 2;   // 스킬 범위(반지름)
    let d = 60;  // 스킬 각도
    let target = vec![vec![0, 1], vec![-1, 1], vec![1, 0], vec![-2, 2]];  // 몬스터들의 좌표
    
    // 정수를 실수로 변환
    let (x, y) = (x as f64, y as f64);
 
    // 범위 내에 있는 몬스터들만 필터링
    let mut intarget: Vec<Vec<i32>> = Vec::new();
    for i in 0..target.len() {
        // 원점으로부터의 거리가 r 이하인 몬스터만 선택
        if distance(target[i][0] as f64, target[i][1] as f64) <= r as f64 {
            intarget.push(target[i].clone());
        }
    }
 
    // 각도를 라디안으로 변환 (60도 = π/3 라디안)
    let d_rad = d as f64 * PI / 180.0;
 
    // 범위 내 몬스터 수 카운트
    let mut count = 0;
 
    // 캐릭터 위치 벡터의 크기
    let u = vector_size(x, y);
    
    // 변수 선언
    let mut v;      // 몬스터 위치 벡터의 크기
    let mut inner;  // 내적 결과
    let mut theta;  // 각도
 
    // 필터링된 몬스터들에 대해 각도 계산
    for t in intarget {
        // 몬스터 위치 벡터의 크기 계산
        v = vector_size(t[0] as f64, t[1] as f64);
        // 두 벡터의 내적 계산
        inner = inner_fn(x, y, t[0] as f64, t[1] as f64);
        // cos⁻¹(내적/(벡터크기1*벡터크기2))로 각도 계산
        theta = (inner / (u * v)).acos();
 
        // 계산된 각도가 주어진 각도의 절반보다 작으면 카운트
        if theta <= d_rad / 2.0 {
            count += 1;
        }
    }
 
    // 결과 출력
    println!("{}", count);
 }