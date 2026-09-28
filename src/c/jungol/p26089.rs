/*
이건 끝까지

케빈 베이컨 수
세상 모든 사람들은 최대 6단계의 관계를 거치면 서로연결
두사람이 친구관계면 두사람의 거리는1
직접 친구가 아니여도 연
*/
//사람의 수
use std::io::{stdin, BufRead, BufReader};

const NMAX: usize = 402;
const MMAX: usize = 10000;
static mut N: usize = 0;
static mut M: usize = 0;
static mut FRONT: usize = 0;
static mut REAR: usize = 0;
static mut ARR: [[usize; NMAX]; NMAX] = [[0; NMAX]; NMAX];
static mut USED: [usize; NMAX] = [0; NMAX];
static mut QUEUE: [usize; NMAX] = [0; NMAX];
static mut DIST: [usize; NMAX] = [0; NMAX];
fn bfs(start: usize) -> usize {
    unsafe {
        FRONT = 0;
        REAR = 0;

        USED = [0; NMAX];
        DIST = [0; NMAX];

        QUEUE[REAR] = start;
        REAR += 1;
        let mut cnt = 0;

        USED[start] = 1;

        while FRONT < REAR {
            let mut current = QUEUE[FRONT];
            FRONT += 1;
            for i in 1..=ARR[current][0] {
                let mut next = ARR[current][i];
                if USED[next] == 1 {
                    continue;
                }
                USED[next] += 1;
                DIST[next] = DIST[current] + 1;
                cnt += DIST[next];

                QUEUE[REAR] = next;
                REAR += 1;
            }
        }

        cnt
    }
}
pub fn main() {
    unsafe {
        let mut reader =BufReader::new(stdin().lock());
        let mut lines = reader.lines();
        let mut first = lines.next().unwrap().unwrap();
        let mut line = first.split_whitespace();
        N = line.next().unwrap().parse::<usize>().unwrap();
        M = line.next().unwrap().parse::<usize>().unwrap();
        for _ in 0..M {
            let second = lines.next().unwrap().unwrap();
            let mut line = second.split_whitespace();
            let a = line.next().unwrap().parse::<usize>().unwrap();
            let b = line.next().unwrap().parse::<usize>().unwrap();
            ARR[a][0] += 1;
            ARR[b][0] += 1;
            ARR[a][ARR[a][0]] = b;
            ARR[b][ARR[b][0]] = a;
        }
        let mut min = MMAX;
        let mut mini = 0;
        for i in 1..=N {
            let sum = bfs(i);
            if sum < min {
                min = sum;
                mini = i;
            }
        }
        println!("{}", mini);
    }
}
