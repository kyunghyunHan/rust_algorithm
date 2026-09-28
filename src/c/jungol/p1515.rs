use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

/*
n개의 구슬 홓ㄹ수

M개의쌍

*/
const MAX: usize = 100;
static mut N: usize = 0;
static mut M: usize = 0;
static mut SRR: [[usize; MAX]; MAX] = [[0; MAX]; MAX];
static mut HRR: [[usize; MAX]; MAX] = [[0; MAX]; MAX];

pub fn example() {
    unsafe {
        // let file = File::open("data.txt").unwrap();
        // let mut reader = BufReader::new(file);
        let mut reader = BufReader::new(stdin().lock());
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
        let mut cnt = 0;
        // for i in 1..=N {
        //     if (HRR[i][0] != N / 2 && SRR[i][0] != N) {
        //         cnt += 1;
        //     }
        // }
        
        println!("{}", cnt);
    }
}
