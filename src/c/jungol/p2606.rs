/*
토마토 초



*/
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
const MAX: usize = 102;
const dy: [i32; 6] = [1, -1, 0, 0, 0, 0];
const dx: [i32; 6] = [0, 0, -1, 1, 0, 0];
const dz: [i32; 6] = [0, 0, 0, 0, 1, -1];
static mut ARR: [[[usize; MAX]; MAX]; MAX] = [[[0; MAX]; MAX]; MAX];
pub fn example() {
    unsafe {
        let file = File::open("data.txt").unwrap();
        let reader = BufReader::new(file);
        let mut line = reader.lines();
        let mut iter = line.next().unwrap().unwrap();
        let mut iter = iter.split_whitespace();
        let (m, n, h) = (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        );
        for i in 1..=h {
            let line = iter.next().unwrap();
            // let mut iter = line.split_whitespace();
            let mut nums = [0usize; MAX];

            for (i, x) in line.split_whitespace().enumerate() {
                nums[i] = x.parse::<usize>().unwrap();
            }
            for i in 0..n {
                ARR[h as usize][m as usize] = nums;
            }
        }

        for i in 1..=h {
            for j in 1..=m {}
        }
    }
}
