/*
순열정복1
스택
*/

use std::io::{BufRead, BufReader};
static mut n: usize = 0;
static mut m: usize = 0;
static mut ARR: [usize; 7] = [0; 7];
static mut USED: [usize; 7] = [0; 7];

fn dfs(top: usize) {
    unsafe {
        if top == m {
            for i in 0..m{
                print!("{}",ARR[i]);
            }
            println!();
            return;
        }

        for i in 1..=n {
            ARR[top] = i;
            dfs(top + 1);
        }
    }
}
pub fn example() {
    unsafe {
        let file = std::fs::File::open("data.txt").unwrap();
        let mut reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let mut line = line.split_whitespace();
            n = line.next().unwrap().parse::<usize>().unwrap();
            m = line.next().unwrap().parse::<usize>().unwrap();
        }
        dfs(0);
    }
}
