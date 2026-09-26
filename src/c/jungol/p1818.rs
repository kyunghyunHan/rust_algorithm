/*
바이러스

dfs로 풀기
bfs로 풀기
*/

/*
순열정복1
스택
*/

use std::io::{BufRead, BufReader};
static mut n: usize = 0;
static mut m: usize = 0;
const MAX: usize = 101;
static mut front: usize = 0;
static mut rear: usize = 0;

static mut ARR: [[usize; MAX]; MAX] = [[0; MAX]; MAX];
static mut USED: [usize; MAX] = [0; MAX];
static mut QUEUE: [usize; MAX] = [0; MAX];

static mut cnt: i32 = 0;
fn bfs(start: usize) {
    unsafe {
        QUEUE[rear] = start;
        USED[start] = 1;
        rear += 1;

        while (front < rear) {
            let mut currnt = QUEUE[front];
            front += 1;

            for i in 1..=ARR[currnt][0] {
                let next = ARR[currnt][i];
                if (USED[next] == 1) {
                    continue;
                }
                USED[next] = 1;
                QUEUE[rear] = next;
                rear += 1;
                cnt += 1;
            }
        }
    }
}
fn dfs(current: usize) {
    unsafe {
        println!("{}", current);
        // dfs(top + 1);
        USED[current] = 1; //현재 컴퓨터를 방문

        for i in 1..=ARR[current][0] {
            //다음 컴퓨터
            let next: usize = ARR[current][i];
            if USED[next] == 1 {
                continue;
            }
            cnt += 1;
            dfs(next);
        }
    }
}
pub fn example() {
    unsafe {
        let file = std::fs::File::open("data.txt").unwrap();
        let mut reader = BufReader::new(file);
        let mut lines = reader.lines();

        n = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        m = lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        //dfs
        for i in 0..m {
            let line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let a = iter.next().unwrap().parse::<usize>().unwrap();
            let b = iter.next().unwrap().parse::<usize>().unwrap();
            ARR[a][0] += 1;
            ARR[b][0] += 1;
            ARR[a][ARR[a][0]] = b;
            ARR[b][ARR[b][0]] = a;
        }
        // dfs(1);
        bfs(1);
        println!("{}", cnt);
    }
}
