use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

/*
n개의 구슬

M개의쌍

*/
const MAX: usize = 100;
static mut N: usize = 0;
static mut M: usize = 0;
static mut SRR: [[usize; MAX]; MAX] = [[0; MAX]; MAX];
static mut HRR: [[usize; MAX]; MAX] = [[0; MAX]; MAX];
static mut USED: [usize; MAX] = [0; MAX];
static mut COUNT: usize = 0;
fn dfs_h(current: usize) {
    unsafe {
        USED[current] = 1;

        for i in 1..=HRR[current][0] {
            let next = HRR[current][i];

            if USED[next] == 1 {
                continue;
            }
            COUNT += 1;
            dfs_h(next);
        }
    }
}
fn dfs_s(current: usize) {
    unsafe {
        USED[current] = 1;

        for i in 1..=SRR[current][0] {
            let next = SRR[current][i];

            if USED[next] == 1 {
                continue;
            }

            COUNT += 1;
            dfs_s(next);
        }
    }
}
pub fn example() {
    unsafe {
        let file = File::open("data.txt").unwrap();
        let mut reader = BufReader::new(file);
        // let mut reader = BufReader::new(stdin().lock());
        let mut line = reader.lines();
        let iter = line.next().unwrap().unwrap();
        let iter = iter.split_whitespace();

        let nm = iter
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        N = nm[0];
        M = nm[1];

        for i in 0..M {
            let iter = line.next().unwrap().unwrap();
            let iter = iter.split_whitespace();
            let nums = iter
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let a = nums[0];
            let b = nums[1];
            HRR[a][0] += 1;
            SRR[b][0] += 1;

            HRR[a][HRR[a][0]] = b;
            SRR[b][SRR[b][0]] = a;
        }
        let mut answer = 0;

        for i in 1..=N {
            USED = [0; MAX];
            COUNT = 0;
            dfs_h(i);
            let light = COUNT;

            USED = [0; MAX];
            COUNT = 0;
            dfs_s(i);
            let heavy = COUNT;

            if light > N / 2 || heavy > N / 2 {
                answer += 1;
            }
        }

        println!("{}", answer);
    }
}
